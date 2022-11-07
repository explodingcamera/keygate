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

- Unless otherwise stated, all files in this repository are licensed under the AGPLv3 License. See the [LICENSE.md](./LICENSE.md) file for details.
- All external contributions are additionally under the [MIT](./LICENSE.MIT.md) license.
- All Community Edition binaries are licensed under the MIT license. See the [LICENSE.md](./LICENSE.MIT.md) file for details.

## FAQ

### How does Keygate compare to other solutions?

A: Keygate is a lightweight, portable, and open source alternative to commercial solutions like Auth0, Okta, or AWS Cognito. A full comparison will be added to the documentation in the future.

### Why Dual License?

The AGPLv3 license was chosen to prevent the projects code being used for propriatary projects. The MIT license is chosen to allow external contributions without the need to sign a CLA. The MIT license is also chosen for the Community Edition binaries to allow the use of Keygate in proprietary projects without the need to open source the entire project.
