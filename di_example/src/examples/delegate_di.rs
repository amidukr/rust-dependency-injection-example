use ambassador::{Delegate, delegatable_trait};

// Lib DI

trait Plugin<C> {
    fn initialize(ctx: &mut C);
}

// Lib Configuration Interface

trait Configuration {
    fn set_run_arguments(&mut self, value: &'static str);
    fn get_run_arguments(&self) -> &'static str;
}

#[delegatable_trait]
trait UseConfiguration {
    type T: Configuration;
    fn configuration(&self) -> &Self::T;
    fn configuration_mut(&mut self) -> &mut Self::T;
}

// Lib Configuration
#[derive(Default)]
struct ConfigurationImpl {
    run_arguments: &'static str,
}

impl Configuration for ConfigurationImpl {
    fn set_run_arguments(&mut self, value: &'static str) {
        self.run_arguments = value;
    }

    fn get_run_arguments(&self) -> &'static str {
        self.run_arguments
    }
}

#[derive(Default)]
struct ConfigurationPlugin {
    configuration: ConfigurationImpl,
}

impl UseConfiguration for ConfigurationPlugin {
    type T = ConfigurationImpl;

    fn configuration(&self) -> &Self::T {
        &self.configuration
    }

    fn configuration_mut(&mut self) -> &mut ConfigurationImpl {
        &mut self.configuration
    }
}

impl<C> Plugin<C> for ConfigurationPlugin {
    fn initialize(_ctx: &mut C) {}
}

// Lib Database Interface

trait DatabaseConnection {
    fn read_query(&self, query: &str);
    fn write_query(&self, query: &str);
}

#[delegatable_trait]
trait UseDatabaseConnection {
    type T: DatabaseConnection;
    fn database_connection(&self) -> &Self::T;
}

// Lib SqlDatabase implementation

trait SqlDatabaseContext: UseConfiguration + UseDatabaseConnectionMut {}

#[delegatable_trait]
trait UseDatabaseConnectionMut: UseDatabaseConnection {
    fn database_connection_mut(&mut self) -> &mut SqlDatabaseConnection;
}

#[derive(Default)]
struct SqlDatabaseConnection {
    connection_string: String,
}

impl DatabaseConnection for SqlDatabaseConnection {
    fn read_query(&self, query: &str) {
        println!("Read {} from {}", query, self.connection_string)
    }

    fn write_query(&self, query: &str) {
        println!("Write {} into {}", query, self.connection_string)
    }
}

#[derive(Default)]
struct SqlDatabasePlugin {
    database_connection: SqlDatabaseConnection,
}

impl UseDatabaseConnection for SqlDatabasePlugin {
    type T = SqlDatabaseConnection;

    fn database_connection(&self) -> &Self::T {
        &self.database_connection
    }
}

impl UseDatabaseConnectionMut for SqlDatabasePlugin {
    fn database_connection_mut(&mut self) -> &mut SqlDatabaseConnection {
        &mut self.database_connection
    }
}

impl<C: SqlDatabaseContext> Plugin<C> for SqlDatabaseConnection {
    fn initialize(ctx: &mut C) {
        ctx.database_connection_mut().connection_string =
            format!("Connection to {}", ctx.configuration().get_run_arguments());
    }
}

// Lib Brokers

// Lib Controller Read

// Lib Controller Write
#[derive(Delegate, Default)]
#[delegate(UseConfiguration, target = "configuration_plugin")]
#[delegate(UseDatabaseConnection, target = "database_connection_plugin")]
#[delegate(UseDatabaseConnectionMut, target = "database_connection_plugin")]
struct ApplicationContext {
    configuration_plugin: ConfigurationPlugin,
    database_connection_plugin: SqlDatabasePlugin,
}

impl SqlDatabaseContext for ApplicationContext {}

const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ApplicationContext>();
};

pub fn run() {
    let mut ctx = ApplicationContext::default();
    ctx.configuration_mut()
        .set_run_arguments("database=host:5555");

    SqlDatabaseConnection::initialize(&mut ctx);

    println!("Settings = {}", ctx.configuration().get_run_arguments());

    ctx.database_connection().read_query("SELECT x FROM table");
    ctx.database_connection()
        .write_query("UPDATE table SET x =1");
}
