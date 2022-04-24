# rust-crud-postgres-app
A Rust CRUD web application with PostgresQL, Rocket server, Diesel DB migration and Handlebars templating engine. Includes image uploads and delete option. 

## Instructions

PostgresQL needs to be installed

You need to initialize a new database "profiles_db" in the root directory of this project as follows:

``initdb /YOUR_PROJECT_PATH/rust-crud-postgres-app/profiles_db``

You then should perform the following:

``cargo build``   
``diesel setup``   
``diesel migration generate profiles_db``   
``pg_ctl -D /YOUR_PROJECT_PATH/rust-crud-template/profiles_db -l logfile start``        
``diesel migration run``    
``cargo run``


###
other instructions soon...
