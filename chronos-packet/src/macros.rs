#[warn(unreachable_patterns)]

#[macro_export]
macro_rules! register_proto {
    {
        $handle_name:ident,
        $(
            $packet_name:ident => ($packet_id:literal, $client_state:ident, $packet_direction:ident),
            $({
                $(
                    $field_name:ident: $field_type:ty
                ),*
            },)?
        )*
    } => {
        $(
            #[derive(Debug, Clone)]
            pub struct $packet_name {
                $($(
                    pub $field_name: $field_type
                ),*)?
            }

            impl Packet for $packet_name {
                fn id(&self) -> i32 {
                    $packet_id
                }

                fn direction(&self) -> PacketDirection {
                    PacketDirection::$packet_direction
                }
            }
        )*

        pub fn $handle_name(state: &ConnectionState, direction: PacketDirection, packet_id: i32, buffer: &mut ByteBuf) -> Option<Box<dyn Packet>> {
            match (state, packet_id) {
                $(
                    (ConnectionState::$client_state, $packet_id) => {
                        let serialized = $packet_name::deserialize(buffer).unwrap();
                        Some(Box::new(serialized))
                    }
                ),*
                _ => None
            }
        }
    };
}