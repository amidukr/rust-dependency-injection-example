use combine_structs::{Fields, combine_fields};
use di_macro::{ContextExtension, FieldEnumerator};

//DI lib

trait Initializable<C> {
    fn init(ctx: &mut C);
}

// Configuration lib
#[derive(Default)]
struct Configuration {
    run_arguments: &'static str,
}

#[allow(dead_code)]
#[derive(Fields, Default)]
struct ConfigurationContextExtension {
    configuration: Configuration,
}

trait UseConfiguration {
    fn configuration(&self) -> &Configuration;
    fn configuration_mut(&mut self) -> &mut Configuration;
}

macro_rules! inject_configuration_impl {
    () => {
        impl UseConfiguration for ApplicationContext {
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
        println!("Init sequence = {}", ctx.configuration().run_arguments);
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
    () => {
        impl UseDatabaseConnection for ApplicationContext {
            type T = PostgresDatabaseConnection;

            fn database_connection(&self) -> &Self::T {
                &self.postgres_database_connection
            }
        }

        impl UsePostgresDatabaseConnection for ApplicationContext {
            fn postgres_database_connection_mut(&mut self) -> &mut PostgresDatabaseConnection {
                &mut self.postgres_database_connection
            }
        }
    };
}

// lib Controller

trait UseReadController {
    fn read_controller(&self) -> &ReadController;
}

trait UseWriteController {
    fn write_controller(&self) -> &WriteController;
}

trait ControllerContext: UseDatabaseConnection + UseReadController + UseWriteController {}

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
    }
}

macro_rules! inject_controller_impl {
    () => {
        impl UseReadController for ApplicationContext {
            fn read_controller(&self) -> &ReadController {
                &self.read_controller
            }
        }

        impl UseWriteController for ApplicationContext {
            fn write_controller(&self) -> &WriteController {
                &self.write_controller
            }
        }

        impl ControllerContext for ApplicationContext {}
    };
}

// Main function and App Context

#[combine_fields(
    ConfigurationContextExtension,
    PostgresDatabaseContextExtension,
    ControllerContextExtension
)]
#[derive(Default, FieldEnumerator)]
struct ApplicationContext {}

inject_postgres_impl!();
inject_controller_impl!();
inject_configuration_impl!();

impl ApplicationContext {
    fn init(&mut self) {
        fn call_init<T: Initializable<ApplicationContext>, F: Fn(ApplicationContext) -> T>(
            ctx: &mut ApplicationContext,
            _closure: F,
        ) {
            T::init(ctx);
        }

        macro_rules! init_callback {
            ($struct_name:ident, $field_name:ident, $listener_type:ident) => {
                call_init(self, |x| x.$field_name);
            };
        }

        enumerate_tags_ApplicationContext_init_listener!(init_callback);
    }
}

const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ApplicationContext>();
};

pub fn run() {
    let mut ctx = ApplicationContext::default();

    ctx.configuration_mut().run_arguments = "DB_URL=127.0.0.1:5555";

    ctx.init();

    ctx.read_controller().do_something(&ctx, "argument");
    ctx.write_controller().do_something(&ctx, "argument");
}
