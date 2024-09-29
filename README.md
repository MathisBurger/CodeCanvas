# CodeCanvas

[![](https://tokei.rs/b1/github/MathisBurger/CodeCanvas?category=lines)](https://github.com/XAMPPRocky/tokei).

CodeCanvas is an online platform to practise programming in an collaborative environment. 
Some tutors can create assignments for you to to and complete. As a registered student you can complete these tasks and get your result graded. 


## Service constellation

![Service Constellation](media/constellation.jpg)


TODO:
- Integrate executor in docker completely
- Build RabbitMQ golang service that initializes all users and queues from a yaml or json config
- No multilpe JOIN requests => Dont show already requested groups under /groups
- Execute diesel migrations on docker container startup