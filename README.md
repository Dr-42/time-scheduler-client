# Time Scheduler

Keep track of your time everyday always

## Usage

Make sure you have [The Server](https://github.com/Dr-42/time-scheduler-server) running

Build the app and run it.

## Building the app

1. Make sure you have rust. My preferred way is to use [rustup](https://rustup.rs/)
2. Make sure you have all the prerequisites for [tauri](https://tauri.app/). Here is the [things you need](https://tauri.app/start/prerequisites/) based on your distro
3. Clone the repo

```sh
git clone https://github.com/Dr-42/time-scheduler-client.git
cd time-scheduler-client
```

4. Install the app

```sh
pnpm install
```

5. Build the app

```sh
pnpm tauri build
```

6. To build the android app

```sh
pnpm tauri android build --apk
```

If you are having code signing issues, refer to this [tauri doc](https://v2.tauri.app/distribute/sign/android/).
