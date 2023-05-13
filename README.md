# README

This project is a demonstration of the organization of the solution when writing a project in a microservice style.

To organize a set of commands into a project, use [Just](https://github.com/casey/just).

---

For demonstration use:

- SSO: [keycloak](https://keycloak.org)
  
- Web-client: yew + [auth2]()
  
- Microservice: [actix-web](https://github.com/actix/actix-web) + [actix-4-jwt-auth](https://github.com/spectare/actix-4-jwt-auth) (there is also a project version with [aliri](https://github.com/erritis/actix-yew-oidc-demo))

---

### Users in keycloak:

| login        | password                         |
|--------------|:--------------------------------:|
| admin        | admin                            |
| react-tester | ^J2WnUY#T3XSi4@AH4NEySqUAhfhid%o |

---

## Deployment

There are two deployment methods:

- [Deployment in docker compose](docs/docker-deployment.md)

- [Deployment to Minikube](docs/minikube-deployment.md)