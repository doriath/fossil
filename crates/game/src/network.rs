use bevy::prelude::*;
use bevy_replicon::prelude::*;
use renet::ServerEvent; // New import
use bevy_replicon_renet::{
    renet::{
        ConnectionConfig, RenetClient, RenetServer,
    },
    netcode::{
        ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication,
        ServerConfig,
    },
    RepliconRenetPlugins,
};
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::time::SystemTime;
use crate::NetworkMode;
use crate::gameplay::{Player, PlayerId};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        // Add Replicon plugins
        app.add_plugins(RepliconPlugins);
        app.add_plugins(RepliconRenetPlugins);

        // Register replicable components
        app.replicate::<Player>();
        app.replicate::<PlayerId>();

        let network_mode = *app.world().resource::<NetworkMode>();

        match network_mode {
            NetworkMode::Server => {
                let server = RenetServer::new(ConnectionConfig::default());
                
                let public_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5000);
                let socket = UdpSocket::bind(public_addr).expect("Failed to bind server socket");
                let server_config = ServerConfig {
                    current_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(),
                    max_clients: 10,
                    protocol_id: 0,
                    public_addresses: vec![public_addr],
                    authentication: ServerAuthentication::Unsecure,
                };
                let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

                app.insert_resource(server);
                app.insert_resource(transport);
                
                info!("Server started on {}", public_addr);
                app.add_systems(Update, server_spawn_player.run_if(
                    |server: Option<Res<RenetServer>>| server.is_some()
                ));
            }
            NetworkMode::Client => {
                let client = RenetClient::new(ConnectionConfig::default());
                
                let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                let client_id = current_time.as_millis() as u64; 
                let server_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5000);
                let socket = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind client socket");
                
                let authentication = ClientAuthentication::Unsecure {
                    client_id,
                    protocol_id: 0,
                    server_addr,
                    user_data: None,
                };
                
                let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

                app.insert_resource(client);
                app.insert_resource(transport);
                
                info!("Client started, connecting to {}", server_addr);
            }
        }
    }
}

fn server_spawn_player(
    mut commands: Commands,
    mut server_events: MessageReader<ServerEvent>, // Changed
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                commands.spawn((
                    Player,
                    PlayerId(*client_id),
                    Replicated,
                    Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(32.0, 32.0)),
                        ..default()
                    },
                    Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                    GlobalTransform::default(),
                ));
                info!("Server spawned player for client {}", client_id);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Client {} disconnected: {:?}", client_id, reason);
            }
        }
    }
}