use combine_structs::{Fields, combine_fields};
use di_macro::{ContextExtension, FieldEnumerator};
use paste::paste;

//DI lib

trait Initializable<C> {
    fn init(ctx: &mut C);
}

macro_rules! enumerate_tags {
    ($ctx:ident, $tag:ident, $callback:ident) => {
        paste! {
        [<enumerate_tags_ $ctx _ $tag >]!($callback)
        }
    };
}

macro_rules! application_context {
    ($ctx: ident) => {
        const _: () = {
            const fn assert_send_sync<T: Send + Sync>() {}
            assert_send_sync::<$ctx>();
        };

        impl Initializable<$ctx> for $ctx {
            fn init(ctx: &mut $ctx) {
                fn call_init<T: Initializable<$ctx>, F: Fn($ctx) -> T>(
                    ctx: &mut $ctx,
                    _closure: F,
                ) {
                    T::init(ctx);
                }

                macro_rules! init_callback {
                    ($struct_name:ident, $field_name:ident, $listener_type:ident) => {
                        call_init(ctx, |x| x.$field_name);
                    };
                }

                enumerate_tags!($ctx, init_listener, init_callback);
            }
        }
    };
}

// Configuration lib
#[derive(Default)]
struct Configuration {
    run_arguments: &'static str,
}

#[allow(dead_code)]
#[derive(Fields, Default, ContextExtension)]
struct ConfigurationContextExtension {
    #[tag(init_listener)]
    configuration: Configuration,
}

trait UseConfiguration {
    fn configuration(&self) -> &Configuration;
    fn configuration_mut(&mut self) -> &mut Configuration;
}

impl<C: UseConfiguration> Initializable<C> for Configuration {
    fn init(ctx: &mut C) {
        println!("Configuration = {}", ctx.configuration().run_arguments)
    }
}

macro_rules! inject_configuration_impl {
    ($ctx:ident) => {
        impl UseConfiguration for $ctx {
            fn configuration(&self) -> &Configuration {
                &self.configuration
            }

            fn configuration_mut(&mut self) -> &mut Configuration {
                &mut self.configuration
            }
        }
    };
}

// lib DB interface

trait DatabaseConnection {
    fn read_query(&self, query: &str);
    fn write_query(&self, query: &str);
}

trait UseDatabaseConnection {
    type T: DatabaseConnection;
    fn database_connection(&self) -> &Self::T;
}

// lib Postgres implementation
#[derive(Default)]
struct PostgresDatabaseConnection {
    connection_string: String,
}

#[allow(dead_code)]
#[derive(Fields, ContextExtension)]
struct PostgresDatabaseContextExtension {
    #[tag(init_listener)]
    postgres_database_connection: PostgresDatabaseConnection,
}

trait UsePostgresDatabaseConnection {
    fn postgres_database_connection_mut(&mut self) -> &mut PostgresDatabaseConnection;
}

impl<C: UseConfiguration + UsePostgresDatabaseConnection> Initializable<C>
    for PostgresDatabaseConnection
{
    fn init(ctx: &mut C) {
        println!(
            "PostgresDB connection init sequence = {}",
            ctx.configuration().run_arguments
        );
        ctx.postgres_database_connection_mut().connection_string =
            format!("Postgres DB on {}", ctx.configuration().run_arguments);
    }
}

impl DatabaseConnection for PostgresDatabaseConnection {
    fn read_query(&self, query: &str) {
        println!("Reading from {}: {}", self.connection_string, query)
    }

    fn write_query(&self, query: &str) {
        println!("Writing into {}: {}", self.connection_string, query)
    }
}

macro_rules! inject_postgres_impl {
    ($ctx: ident) => {
        impl UseDatabaseConnection for $ctx {
            type T = PostgresDatabaseConnection;

            fn database_connection(&self) -> &Self::T {
                &self.postgres_database_connection
            }
        }

        impl UsePostgresDatabaseConnection for $ctx {
            fn postgres_database_connection_mut(&mut self) -> &mut PostgresDatabaseConnection {
                &mut self.postgres_database_connection
            }
        }
    };
}

// lib Oracle

#[allow(dead_code)]
#[derive(Fields, ContextExtension)]
struct OracleDatabaseContextExtension {}

macro_rules! inject_oracle_impl {
    ($ctx: ident) => {
        impl DatabaseConnection for $ctx {
            fn read_query(&self, query: &str) {
                println!("Reading from Oracle DB: {}", query)
            }

            fn write_query(&self, query: &str) {
                println!("Writing into Oracle DB {}", query)
            }
        }

        impl UseDatabaseConnection for $ctx {
            type T = $ctx;

            fn database_connection(&self) -> &Self::T {
                self
            }
        }
    };
}

// lib Publisher

#[allow(dead_code)]
#[derive(Default, Fields, ContextExtension)]
struct PublisherExtension {}

trait BrokerSender {
    fn send_to_broker(&self, value: &str);
}

trait Publisher {
    fn publish(&self, value: &str);
}

trait UsePublisher {
    type T: Publisher;
    fn publisher(&self) -> &Self::T;
}

macro_rules! inject_publisher_impl {
    ($ctx:ident) => {
        impl Publisher for $ctx {
            fn publish(&self, value: &str) {
                macro_rules! broker_callback {
                    ($struct_name:ident, $field_name:ident, $listener_type:ident) => {
                        self.$field_name.send_to_broker(value);
                    };
                }

                enumerate_tags!($ctx, broker, broker_callback);
            }
        }

        impl UsePublisher for $ctx {
            type T = $ctx;

            fn publisher(&self) -> &Self::T {
                self
            }
        }
    };
}

// lib RabbitMq broker

#[allow(dead_code)]
#[derive(Default, Fields, ContextExtension)]
struct RabbitMqContextExtension {
    #[tag(broker)]
    rabbit_mq: RabbitMq,
}

#[derive(Default)]
struct RabbitMq;

impl BrokerSender for RabbitMq {
    fn send_to_broker(&self, value: &str) {
        println!("{} sent to RabbitMq", value);
    }
}

macro_rules! inject_rabbit_mq_impl {
    ($ctx:ident) => {};
}

// lib Kafka broker

#[allow(dead_code)]
#[derive(Default, Fields, ContextExtension)]
struct KafkaContextExtension {
    #[tag(broker)]
    kafka: Kafka,
}

#[derive(Default)]
struct Kafka;

impl BrokerSender for Kafka {
    fn send_to_broker(&self, value: &str) {
        println!("{} sent to Kafka", value);
    }
}

macro_rules! inject_kafka_impl {
    ($ctx:ident) => {};
}

// lib Controller

trait UseReadController {
    fn read_controller(&self) -> &ReadController;
}

trait UseWriteController {
    fn write_controller(&self) -> &WriteController;
}

trait ControllerContext:
    UseDatabaseConnection + UseReadController + UseWriteController + UsePublisher
{
}

#[allow(dead_code)]
#[derive(Fields)]
struct ControllerContextExtension {
    read_controller: ReadController,
    write_controller: WriteController,
}

#[derive(Default)]
struct ReadController {}

impl ReadController {
    fn do_something<C: ControllerContext>(&self, ctx: &C, argument: &str) {
        ctx.database_connection()
            .read_query(format!("SELECT * FROM table WHERE id = '{}'", argument).as_str());
    }
}

#[derive(Default)]
struct WriteController {}

impl WriteController {
    fn do_something<C: ControllerContext>(&self, ctx: &C, argument: &str) {
        ctx.database_connection().write_query(
            format!("UPDATE table SET value = 'new' WHERE id = '{}'", argument).as_str(),
        );
        ctx.publisher()
            .publish(format!("WriteController '{}'", argument).as_str());
    }
}

macro_rules! inject_controller_impl {
    ($ctx:ident) => {
        impl UseReadController for $ctx {
            fn read_controller(&self) -> &ReadController {
                &self.read_controller
            }
        }

        impl UseWriteController for $ctx {
            fn write_controller(&self) -> &WriteController {
                &self.write_controller
            }
        }

        impl ControllerContext for $ctx {}
    };
}

// Main function and App Context

#[combine_fields(
    ConfigurationContextExtension,
    PostgresDatabaseContextExtension,
    ControllerContextExtension,
    PublisherExtension,
    RabbitMqContextExtension,
    KafkaContextExtension
)]
#[derive(Default, FieldEnumerator)]
struct ApplicationProfile1 {}

application_context!(ApplicationProfile1);
inject_postgres_impl!(ApplicationProfile1);
inject_controller_impl!(ApplicationProfile1);
inject_configuration_impl!(ApplicationProfile1);
inject_publisher_impl!(ApplicationProfile1);
inject_rabbit_mq_impl!(ApplicationProfile1);
inject_kafka_impl!(ApplicationContext);

#[combine_fields(
    ConfigurationContextExtension,
    OracleDatabaseContextExtension,
    ControllerContextExtension,
    PublisherExtension,
    RabbitMqContextExtension
)]
#[derive(Default, FieldEnumerator)]
struct ApplicationProfile2 {}

application_context!(ApplicationProfile2);
// Oracle is used in Profile2
inject_oracle_impl!(ApplicationProfile2);
inject_controller_impl!(ApplicationProfile2);
inject_configuration_impl!(ApplicationProfile2);
inject_publisher_impl!(ApplicationProfile2);
// No kafka broker used in Profile2
inject_rabbit_mq_impl!(ApplicationProfile2);

fn do_run<T: Initializable<T> + Default + UseConfiguration + ControllerContext>() {
    let mut ctx = T::default();

    ctx.configuration_mut().run_arguments = "DB_URL=127.0.0.1:5555";

    T::init(&mut ctx);

    ctx.read_controller().do_something(&ctx, "argument");
    ctx.write_controller().do_something(&ctx, "argument");
}

// Example Output
//
// Running Profile1
// Configuration = DB_URL=127.0.0.1:5555
// PostgresDB connection init sequence = DB_URL=127.0.0.1:5555
// Reading from Postgres DB on DB_URL=127.0.0.1:5555: SELECT * FROM table WHERE id = 'argument'
// Writing into Postgres DB on DB_URL=127.0.0.1:5555: UPDATE table SET value = 'new' WHERE id = 'argument'
// WriteController 'argument' sent to RabbitMq
// WriteController 'argument' sent to Kafka

// Running Profile2
// Configuration = DB_URL=127.0.0.1:5555
// Reading from Oracle DB: SELECT * FROM table WHERE id = 'argument'
// Writing into Oracle DB UPDATE table SET value = 'new' WHERE id = 'argument'
// WriteController 'argument' sent to RabbitMq

pub fn run() {
    println!("Running Profile1");
    do_run::<ApplicationProfile1>();
    println!();

    println!("Running Profile2");
    do_run::<ApplicationProfile2>();
}
