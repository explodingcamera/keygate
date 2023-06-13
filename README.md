<div align=left>
  <h1 align=left><a href='https://keygate.io'>keygate.io</a>
  </h1>
</div>

> Open Source User Identity Managment and Authentication

<br/>

## Status

Keygate is currently in a very early stage of development. It is not ready for production use.

<br/>

## Goals

- **Not too much choice**
  Keygate should be easy to use and easy to understand. Not a lot of options, but the right ones.
- **Easy to deploy**
  Keygate should be easy to deploy. Single binary, no database, no configuration file needed.
  https://fishshell.com/docs/current/design.html#configurability-is-the-root-of-all-evil
- **Portable**
  Data should be portable. You should be able seamlessly move your user data from keygate to your own solution. No vendor lock-in. No data lock-in.
- **Privacy**
  Keygate should not collect any data that is not needed for the service to work. No analytics, no tracking.
- **Minimal features**
  Focusing on the core features of user identity management and authentication. Additional features should only provide the bare minimum functionality, and should be easy to replace with your own implementation or external service (e.g a custom access control or OAuth2 solution).
- **Secure**
  Keygate should be secure by default. It should be easy to use and hard to misuse. Insecure login methods like SMS are not supported.
- **Flexibility**
  It should be easy to build your own service on top of keygate. Keygate should be flexible enough to be used in a variety of use cases, but not too flexible to be misused. It is not the goal to build a general purpose solution for all possible use cases.
- **Open Source**
  Keygate should be open source. It should be easy to contribute to the project. It should be easy to audit the code. It should be easy to extend the project with your own features.

<br/>

## License

- Unless otherwise stated, all files in this repository are licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0).

## Contributing

Unless otherwise stated, any contribution intentionally submitted for inclusion in the work by you, shall be licensed as above without any additional terms or conditions.

## FAQ

### How does Keygate compare to other solutions?

A: Keygate is a lightweight, portable, and open source alternative to commercial solutions like Auth0, Okta, or AWS Cognito. A full comparison will be added to the documentation in the future.
