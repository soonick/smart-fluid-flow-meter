# Backend

The back-end for `smart-fluid-flow-meter`. Written in Rust.

Receives measures from the meter and stores them in a database.

## Run

To start the development version:

```
make start
```

If you want to run a version that includes a local database, you can use:

```
make start-mysql
```

## Tests

Sadly, firestore requires a service account key to work even when connecting to an emulator, so we need to create one from [google cloud console](https://console.cloud.google.com/iam-admin/serviceaccounts). It's not necessary to give any permissions to the service account. Save the json key to a file named `service-account-key.json` in the same directory as this README file.

To run tests:

```
make test
```
