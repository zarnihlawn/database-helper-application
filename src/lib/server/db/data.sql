INSERT INTO content_type (name, description) VALUES ('md', "Markdown is a lightweight markup language using plain text syntax. It's designed for easy reading and writing, and converts to HTML, making it ideal for web content.");
INSERT INTO content_type (name, description) VALUES ('sql', "SQL is a powerful language for managing and manipulating data in relational databases. It allows for querying, updating, and defining data structures efficiently.");
INSERT INTO content_type (name, description) VALUES ('json', "JSON (JavaScript Object Notation) is a lightweight, text-based data interchange format. It's human-readable, easily parsed by machines, and widely used for web data.");

INSERT INTO user_authentication_type (type, description) VALUES ('guest', 'Empty authentication type, your workspaces will be public and can be access by other users on this device.');
INSERT INTO user_authentication_type (type, description) VALUES ('email and password', 'Basic authentication type, it will grant you your own private workspace.');
INSERT INTO user_authentication_type (type, description) VALUES ('email verificated', 'Advanced authentication type, it will allow you to collaborate with other.');

INSERT INTO datasource (type, description) VALUES ('PostgreSQL', "PostgreSQL: a robust, open-source RDBMS, excels with complex queries, strong SQL compliance, and advanced features like transactions and extensibility, ideal for data-intensive applications.");
INSERT INTO datasource (type, description) VALUES ('SQLite', "SQLite: a serverless, file-based SQL database, embedded within applications, offering lightweight, reliable data storage with strong SQL support, ideal for local data management.");
INSERT INTO datasource (type, description) VALUES ('MongoDB', "MongoDB: a flexible, NoSQL document database, utilizing JSON-like BSON, designed for high scalability and agile development, ideal for handling unstructured or semi-structured data.");
INSERT INTO datasource (type, description) VALUES ('Oracle', "Oracle: a robust, enterprise-grade RDBMS, known for scalability, reliability, and advanced features, supporting complex transactions and data warehousing, with a focus on mission-critical applications.");
INSERT INTO datasource (type, description) VALUES ('MySQL', "MySQL: a widely used, open-source relational database, known for its speed and reliability, supporting diverse applications from web development to enterprise data storage.");

INSERT INTO datasource_authentication_type (type, description) VALUES ('User and Password', "Simplest way of database authentication.");

