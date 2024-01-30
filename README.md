# homework_3
Instruction

A- For the front-end part

<h1> If we write a city name as an input and search, then related weather info of the city and its weather condition image DYNAMICALLY will be shown. </h1>

1- Main page

![1](https://github.com/furkancetinalp/icp_bootcamp_homework_three/assets/99509540/3aa29c4f-8752-4036-ba03-e72d03a1a0c7)

2- Weather data of Dublin => clear sky

![2_clear_sky](https://github.com/furkancetinalp/icp_bootcamp_homework_three/assets/99509540/ab0a936a-8e37-4c12-8f4b-f6856ef5de3b)

3- Weather data of Tokio => partly cloud

![3_partly_cloud](https://github.com/furkancetinalp/icp_bootcamp_homework_three/assets/99509540/200648ff-26b7-4b60-bfd7-214ef55a9cc9)

4- Weather data of Stockholm => rain

![4_rain](https://github.com/furkancetinalp/icp_bootcamp_homework_three/assets/99509540/bf64463c-2bd5-4e4a-82e4-c14f5bd5b9d9)

5- Weather data of Puvirnituq (a willage in Canada) => snow

![snow_again](https://github.com/furkancetinalp/icp_bootcamp_homework_three/assets/99509540/51bcd827-bdc7-4765-b95f-a51dd9d255e9)

6- We can search specific cities by clicking which are shown on the right side

![california](https://github.com/furkancetinalp/icp_bootcamp_homework_three/assets/99509540/a9701bb7-c32a-4737-bf9c-1a8a10d594c8)

![7_wigdet_again](https://github.com/furkancetinalp/icp_bootcamp_homework_three/assets/99509540/f4d755aa-180a-4e82-807e-a1e6f5f291d7)

7- If we search for an invalid ciy, result will be similar to this picture:

![8_invalid_city](https://github.com/furkancetinalp/icp_bootcamp_homework_three/assets/99509540/9f66693c-582b-4962-b28c-5b284a4e6f4d)


B- Backend canister

![backend_canister](https://github.com/furkancetinalp/icp_bootcamp_homework_three/assets/99509540/8e4adb0b-a68e-4b38-8fd8-553fa934ab59)



Welcome to your new homework_3 project and to the internet computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with homework_3, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd homework_3/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
