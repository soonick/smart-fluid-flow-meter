# Backend

The back-end for `smart-fluid-flow-meter`. Written in Rust.

Receives measures from the meter and stores them in a database.

## Run

The easiest way to run locally is to use:

```
make start-mysql
```

This command uses docker compose to start a MySQL container with the latest schema and another container running the backend. The backend will use `.env.sample` for configuration.

If you need more granular control over the settings, create your own `.env` file:

```
cp .env.sample .env
```

Update the configurations accordingly and start the service.

```
make start
```

## Tests

Sadly, firestore requires a service account key to work even when connecting to an emulator, so we need to create one from [google cloud console](https://console.cloud.google.com/iam-admin/serviceaccounts). It's not necessary to give any permissions to the service account. Save the json key to a file named `service-account-key.json` in the same directory as this README file.

To run tests:

```
make test
```

## Production

This section pertains to the official deployment of this service. It can be safely ignored by most people.

## Connecting to production DB

The official deployment uses Firestore for storage. To run the backend locally and connect to the production database we need an Google Cloud JSON key that allows to write access to the correct database.

Put the key in a file named `service-account-key.json` in the same folder as this README file and run using this command:

```
make start-prod
```
