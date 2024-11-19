# CodeCanvas

[![](https://tokei.rs/b1/github/MathisBurger/CodeCanvas?category=lines)](https://github.com/XAMPPRocky/tokei).

CodeCanvas is an online platform to practise programming in an collaborative environment. 
Some tutors can create assignments for you to to and complete. As a registered student you can complete these tasks and get your result graded. 
The application has built in code execution which makes it easy for tutors and students to validate the code handed in for the assignments. Furthermore, question based assignments are supported too. This makes it easy for tutors to not only create coding assignments, but also assignments where students have to answer predefined questions.

## Getting started

We do not recommend to host the application yourself, altough you can do it. If you want to start using this app for your small group, fell free to create an account on [our Service](https://code-canvas.app).



## Setup 

1. Take the `sample-compose.yml` file and fill out the missing values. 
2. Create the docker compose cluster
3. Start the postgres and mongodb and create following entries:

| Name               | Beschreibung                                                                                                                        |
|--------------------|-------------------------------------------------------------------------------------------------------------------------------------|
| Usernator Postgres | Create a postgres user for the usernator service and assign a database to the user.                                                 |
| Executor Postgres  | Create a postgres user for the executor and assign a database to the user.                                                          |
| Executor MongoDB   | Just create a mongoDB user for the executor alongside with a database. Grant only read permissions to the executor user.            |
| Tasky Postres      | Create a postgres user for tasky service and assign a database to the user.                                                         |
| Tasky MongoDB      | Create a mongoDB user for tasky and grant the user read/write access to the shared database with the executor service.              |

4. Start the cluster again.
5. Now you can initialize the web container with env `API_URI` and `EXECUTOR_UI_URL`.
6. Ensure that the executor UI is protected by a password proxy or something else.
7. Enjoy using your application.

## Contribution

This project is completely community driven. Therefore, everyone can contribute to this project and help to improve the project.
If you want to give some monetary compensation for the work we are doing, feel free to do so. There is currently no option to do that.
Maybe also check [CONTRIBUTING.md](CONTRIBUTING.md)

## NOTICE: Data fetching performance

Please notice that the application grew over time. There were no thoughts about the database schema before creation. Furthermore, diesel.rs as the library for database interactions was pretty new to me, which is the reason the data fetching is not optimal. Some mistakes in the schema have already been fixed and improved, but the data fetching is still very imperformant. Altough storing big amounts of data is no problem anymore, fetching might take a little longer, but is statically limited due to the pagination. So the application will work at constant speed with constant response times, but executes more SQL queries than nessesary. This is a known bug, but we do not want to face it currently, because it is not a problem yet. We might take a deeper look into it, when the platform gains more users and we want to reduce load on the application to save server costs. Furthermore, this feature might be faced with some good caching policies in the future.
