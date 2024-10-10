# CodeCanvas

[![](https://tokei.rs/b1/github/MathisBurger/CodeCanvas?category=lines)](https://github.com/XAMPPRocky/tokei).

CodeCanvas is an online platform to practise programming in an collaborative environment. 
Some tutors can create assignments for you to to and complete. As a registered student you can complete these tasks and get your result graded. 


## Setup 

1. Take the `sample-compose.yml` file and fill out the missing values. 
2. Create the docker compose cluster
3. Start the postgres, mongodb and rabbitmq container and create following entries:

| Name               | Beschreibung                                                                                                                        |
|--------------------|-------------------------------------------------------------------------------------------------------------------------------------|
| Usernator Postgres | Create a postgres user for the usernator service and assign a database to the user.                                                 |
| Usernator RabbitMQ | Create an rabbitMQ user for the usernator. Create a topic `global_create_user` where the usernator user has access to publish.      |
| Executor Postgres  | Create a postgres user for the executor and assign a database to the user.                                                          |
| Executor RabbitMQ  | Just create an user for the executor and grant permissions to `global_create_user` topic. The queue will be created automatically.  |
| Executor MongoDB   | Just create a mongoDB user for the executor alongside with a database. Grant only read permissions to the executor user.            |
| Tasky Postres      | Create a postgres user for tasky service and assign a database to the user.                                                         |
| Tasky MongoDB      | Create a mongoDB user for tasky and grant the user read/write access to the shared database with the executor service.              |

4. Start the cluster again.
5. Now you can initialize the web container with env `API_URI` and `EXECUTOR_UI_URL`.
6. Ensure that the executor UI is protected by a password proxy or something else.
7. Enjoy using your application.