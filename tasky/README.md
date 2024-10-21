# Tasky

Tasky handles all activities and events regarding groups, assignments and solutions.


## Testing coverage

The aim of our tests is to ensure the end-to-end functionality of the microservice. Therefore, we do not test the repositories and other activities.
We only test seperate modules like the security and the whole endpoint functionality. Through this methodology we ensure the functionality of out application
end to end without the effort of testing each individual component like the models or mongoDB.

**NOTE: There is no test coverage for assignment test/question creation and anything going further than that (solutions, etc.). Its kind of complicated to implement tests for these cases and therefore, they are left our for now. There will be test cases for these functionality later.**
