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

impl DatabaseConnection for PostgresDatabaseConnection {
    fn read_query(&self, query: &str) {
        println!("Reading from Postgres DB: {}", query)
    }

    fn write_query(&self, query: &str) {
        println!("Writing into Postgres DB: {}", query)
    }
}

// lib Controller

trait UseReadController {
    fn read_controller(&self) -> &ReadController;
}

trait UseWriteController {
    fn write_controller(&self) -> &WriteController;
}

trait ControllerContext: UseDatabaseConnection + UseReadController + UseWriteController {}

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

// Main function and App Context

#[derive(Default)]
struct ApplicationContext {
    read_controller: ReadController,
    write_controller: WriteController,
    postgres_database_connection: PostgresDatabaseConnection,
}

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

impl UseDatabaseConnection for ApplicationContext {
    type T = PostgresDatabaseConnection;

    fn database_connection(&self) -> &Self::T {
        &self.postgres_database_connection
    }
}

impl ControllerContext for ApplicationContext {}

const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ApplicationContext>();
};

pub fn run() {
    let ctx = ApplicationContext::default();

    ctx.read_controller().do_something(&ctx, "argument");
    ctx.write_controller().do_something(&ctx, "argument");
}
