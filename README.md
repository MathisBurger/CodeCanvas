# CodeCanvas

[![](https://tokei.rs/b1/github/MathisBurger/CodeCanvas?category=lines)](https://github.com/XAMPPRocky/tokei)

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

## Development status

This project is now in its final phase, with all required features fully implemented. At this point, there is no concrete roadmap for further development, and the active development process for this application has been concluded.

We will continue to address feature requests and bug reports on a case-by-case basis, implementing new features or fixes only if they align with our vision and appear valid. However, we will not introduce additional features independently.

Our primary goal is to maintain the application’s simplicity and ease of use. Adding too many features could compromise this simplicity.

We encourage you to share any feature requests or bug reports, but please understand that new features will only be added if driven by clear external needs.

## NOTICE: Data fetching performance

Please note that the application has evolved over time, and no database schema was initially planned. Additionally, at the time of development, I was relatively new to using Diesel.rs as the library for database interactions. This has resulted in suboptimal data fetching. While some issues in the schema have already been addressed and improved, the current data fetching process remains inefficient.

Although storing large amounts of data is no longer an issue, data retrieval can still take longer than desired. However, thanks to pagination, response times are statically limited and remain consistent, regardless of the dataset size. The downside is that the application executes more SQL queries than necessary. This inefficiency is a known issue, but it is not a priority at the moment, as it doesn't pose a significant problem yet.

We plan to revisit this aspect when the platform gains more users and reducing server load becomes crucial to minimize operational costs. In the future, this inefficiency could potentially be mitigated through the implementation of effective caching strategies.

## NOTICE: Code quality and improvement

This code may not meet the expectations of those more experienced with Rust. When I first started working on this project, my understanding of Rust was still developing. Over the course of the project, I’ve learned a great deal, but as a result, the code may not fully adhere to best practices. I kindly ask for your understanding when reviewing it. Even now, as I revisit this project, I can see several areas where I would approach things differently.

There may be some improvements in the future, but even after them, there will be some huge chunks of bad code from the perspective of an experienced rust developer.
