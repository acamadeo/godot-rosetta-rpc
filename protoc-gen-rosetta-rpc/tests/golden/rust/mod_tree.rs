pub mod rosetta {
    pub mod example {
        include!("rosetta/example/clock.rs");
        include!("rosetta/example/clock_descriptors.rs");
        include!("rosetta/example/clock_client.rs");
        include!("rosetta/example/clock_adapter.rs");
        include!("rosetta/example/game_service.rs");
        include!("rosetta/example/game_service_descriptors.rs");
        include!("rosetta/example/game_service_client.rs");
        include!("rosetta/example/game_service_adapter.rs");
        include!("rosetta/example/service_factory.rs");
        include!("rosetta/example/bootstrap.rs");
    }
}
