# CodeCanvas

[![](https://tokei.rs/b1/github/MathisBurger/CodeCanvas?category=lines)](https://github.com/XAMPPRocky/tokei).

CodeCanvas is an online platform to practise programming in an collaborative environment. 
Some tutors can create assignments for you to to and complete. As a registered student you can complete these tasks and get your result graded. 

**NOTE: Currently there is an QuestionBased option for Assignments, but it does not work yet properly. It is not implemented yet.**

## Service constellation

![Service Constellation](media/constellation.jpg)


TODO:
- Integrate executor in docker completely
- Build golang service that initializes all users and queues from a yaml or json config for RabbitMQ, postgres and MongoDB
- Execute diesel migrations on docker container startup
- Optimize tasky SQL requests with JOINs