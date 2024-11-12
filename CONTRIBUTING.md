# Contribution guidelines for CodeCanvas

Hi, thank you for your will to comtribute to this project.
Let me first of all give you an good overview over this project.

It constists of five microservices:


### Authy

Authy is the main auth proxy. It recieves all requests and proxys them to the microservice they belong.
Furthermore, some requests may need authorization. Authy provides the `X-CodeCanvas-UserId` and `X-CodeCanvas-UserRoles` headers with each request to indicate what user executed
this request. Authy also handles JWt creation and validation as well as the bug report feature for the frontend. 

If you take a look at the configuration of this microservice, you will see, that there are dynamic service location configurations as well as configuration for URI white and blacklists. Whitelists mean these endpoints are unauthorized and will be proxied without auth. The other configuration options explain theirselves by name.

### Executor

Executor is our code execution engine. It is based on tork.run engine. We built our own task generation and REST-API on top of it to ensure it integrates with our custom images we use for code execution. 

### Tasky 

Tasky could be named as the main service of this application. It handles most of the business logic like groups, assignments, solutions, comments on solutions, etc. 
It is also the most complex microservice. If you start working with it, make sure you set following environment variable `IS_DOCKER=false` and set the symlinks for the protobuf implementation properly. This will be further explained in the dev-setup section.


### Usernator

Usernator microservice handles all user related transactions like creating new users, tutors, and login.

### Web

This is the web component of the application that implements our main user interface.


## Specific files

As you might have noticed, there are some things arround that might not seem straight forward. 

There are three docker compose files:
- `docker-compose.yml` used for local dev setup (mac only)
- `sample-compose.yml` used for prod setup
- `ci-compose.yml` used for testing purposes in the CI environment

The `update.sh` file is used for updating the prod environment.
The `pipeline-tools` directory contains a custom written golang tool for our CI/CD flow. 


## Setup instructions

First of all, you have to set all the symlinks properly. 

```shell
ln ./usernator/api.proto ./tasky/api.proto
ln ./tasky/tasky.proto ./usernator/tasky.proto
ln ./tasky/tasky.proto ./executor/tasky.proto
```

To start your dev environment you can now just run `docker-compose up`.
**NOTE:** For local development with tasky (outside of docker) you will need to set `IS_DOCKER=false` environment variable.

Happy coding!

## Behavioural guidelines


### 1. **Be Respectful**
   - Treat everyone with respect and kindness. Different people have different backgrounds, experiences, and perspectives.
   - Engage in constructive feedback, and avoid any personal attacks or disrespectful behavior.
   - Be mindful of how your words might be interpreted by others and strive to create an inclusive, supportive community.

### 2. **Follow the Code of Conduct**
   - All contributors are expected to follow our [Code of Conduct](CODE_OF_CONDUCT.md), which sets out the expectations for interaction and behavior within the project.
   - If you observe unacceptable behavior or interactions, please report it through the appropriate channels.

### 3. **Communication**
   - Be clear and concise when reporting issues, requesting features, or submitting pull requests.
   - Use proper language and be polite. If you disagree with feedback, do so respectfully and professionally.
   - In discussions, focus on the issue or topic at hand and avoid unnecessary personal comments or off-topic remarks.

### 4. **Commit Messages**
   - Everyone has to follow the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) specification to ensure good commit messages.

### 5. **Be Inclusive**
   - Avoid using language that may be discriminatory or exclusive. Use gender-neutral and inclusive language in your contributions.
   - Be sensitive to different cultural, social, and regional contexts when discussing or suggesting changes.

### 6. **No Spam or Promotional Content**
   - Avoid submitting irrelevant, spammy, or promotional content. Contributions should be focused on improving the project.
   - Any irrelevant links, advertisements, or solicitations will be removed.

### 7. **Respect Project Guidelines**
   - Familiarize yourself with the project’s guidelines and coding standards before contributing.
   - Ensure that your pull requests are aligned with the project’s goals and design principles.
   - Review the project’s documentation and issues before submitting new contributions to avoid duplication.

### 8. **Ask for Help If Needed**
   - If you're unsure about how to proceed, ask! We're here to help you. Whether it’s about how to submit a pull request, how to implement a feature, or how to solve a specific issue, don’t hesitate to reach out.

### 9. **Review Process**
   - Contributions are not automatically accepted. They will be reviewed by maintainers and may require changes or updates before they are merged.
   - Maintain an open and receptive attitude toward feedback. You might need to make adjustments to your contribution based on the review.

### 10. **Respect the Maintainers' Time**
   - Be patient when waiting for a response to your pull request or issue. Maintainers often have many responsibilities and may not be able to respond immediately.
   - Ensure that your contributions are well-tested, documented, and complete, to minimize back-and-forth during the review process.

### 11. **Code style**
   - Always make sure you ran prettier when commiting to the web component. 
   - We should follow the clippy instructions for rust modules
   - Please use an code formatter for your rust modules. We use rustfmt here.  

---

By following these guidelines, we aim to maintain a welcoming and productive environment for all contributors. Thank you for helping us make CodeCanvas better!