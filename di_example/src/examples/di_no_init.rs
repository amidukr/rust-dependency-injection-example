use combine_structs::{Fields, combine_fields};

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
struct PostgresDatabaseConnection {}

#[allow(dead_code)]
#[derive(Fields)]
struct PostgresDatabaseContextExtension {
    postgres_database_connection: PostgresDatabaseConnection,
}

impl DatabaseConnection for PostgresDatabaseConnection {
    fn read_query(&self, query: &str) {
        println!("Reading from Postgres DB: {}", query)
    }

    fn write_query(&self, query: &str) {
        println!("Writing into Postgres DB: {}", query)
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

#[combine_fields(PostgresDatabaseContextExtension, ControllerContextExtension)]
#[derive(Default)]
struct ApplicationContext {}

inject_postgres_impl!();
inject_controller_impl!();

const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ApplicationContext>();
};

pub fn run() {
    let ctx = ApplicationContext::default();

    ctx.read_controller().do_something(&ctx, "argument");
    ctx.write_controller().do_something(&ctx, "argument");
}
