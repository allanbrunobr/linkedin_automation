# LinkedIn Automation

This project automates the process of scheduling and publishing posts on LinkedIn. It includes both a frontend (built with React) and a backend (built with Rust). The backend handles post scheduling and publishing using LinkedIn's API, while the frontend allows users to schedule posts through a simple interface.

## Features

- **Post Scheduling**: Schedule LinkedIn posts to be published at a specified date and time.
- **Post Management**: View, edit, and delete scheduled posts.
- **LinkedIn Integration**: Seamlessly publish posts on LinkedIn using their API.
- **User-Friendly Interface**: An intuitive React frontend for managing posts.

## Project Structure

The project is organized into two main directories:

- `frontend/`: Contains the React application.
- `backend/`: Contains the Rust application that handles scheduling, post management, and communication with LinkedIn's API.

## Getting Started

### Prerequisites

- **Node.js**: Install Node.js and npm for the frontend.
- **Rust**: Install Rust for the backend.
- **MongoDB**: Install MongoDB, as it's used to store scheduled posts.

### Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/yourusername/linkedin_automation.git
   cd linkedin_automation
   ```

2. **Setup the Frontend:**

   ```bash
   cd frontend
   npm install
   npm start
   ```

   This will start the React application on `http://localhost:3000`.

3. **Setup the Backend:**

   ```bash
   cd backend
   cargo build
   cargo run --bin web_server
   ```

   This will start the Rust web server on `http://localhost:8080`.

### Configuration

This project uses a `config.toml` file for storing configuration settings. The file should be placed in the root directory of the `backend` and include the following parameters:

```toml
client_id = "YOUR_CLIENT_ID"
client_secret = "YOUR_CLIENT_SECRET"
redirect_uri = "http://localhost:3000/callback"
access_token = "YOUR_ACCESS_TOKEN"
```
### Environment Variables

In addition to the `config.toml` file, this project requires an environment variable to be set:

- **`TIMEZONE_OFFSET`**: This variable is mandatory and should be set to the number of hours offset from UTC for your timezone. For example, for Brazil Standard Time (UTC-3), you would set it to `-3`.

You can set this variable in a `.env` file in the root of your backend directory:

```plaintext
TIMEZONE_OFFSET=-3
```


### Obtaining a LinkedIn Access Token

To get an `access_token` from LinkedIn, you need to follow these steps:

1. Register your application with LinkedIn to get a `client_id` and `client_secret`. You can do this through the [LinkedIn Developer Portal](https://www.linkedin.com/developers/apps).

2. Implement the OAuth 2.0 authorization flow to get an authorization code and then exchange it for an `access_token`.

   For a detailed guide, refer to LinkedIn's [OAuth 2.0 documentation](https://docs.microsoft.com/en-us/linkedin/shared/authentication/authentication?context=linkedin%2Fcontext).

3. Use the obtained `access_token` to configure your `config.toml` file.

**Important:** Keep your `config.toml` file secure and do not share it publicly, as it contains sensitive information.

### Running the Application

- **Frontend:** Start the frontend by running `npm start` in the `frontend` directory.
- **Backend:** Start the backend by running `cargo run --bin web_server` in the `backend` directory.

The frontend will be available at `http://localhost:3000` and the backend API at `http://localhost:8080`.

## API Endpoints

The backend exposes several API endpoints:

- **POST /schedule**: Schedule a new post.
- **GET /posts**: Retrieve all scheduled posts.
- **DELETE /posts/:id**: Delete a scheduled post.

## MongoDB Setup

Ensure MongoDB is running, and the necessary database and collections are created:

```bash
use lkdin-posts;

db.createCollection("posts", {
    validator: {
        $jsonSchema: {
            bsonType: "object",
            required: ["title", "content", "scheduled_time"],
            properties: {
                title: {
                    bsonType: "string",
                    description: "Title of the post"
                },
                content: {
                    bsonType: "string",
                    description: "Content of the post"
                },
                scheduled_time: {
                    bsonType: "date",
                    description: "Scheduled time for the post"
                },
                status: {
                    bsonType: "string",
                    description: "Status of the post (e.g., pending, published)"
                }
            }
        }
    }
});
```

## Development

### Frontend

The frontend is a React application that uses Bootstrap for styling. It allows users to create, view, and delete scheduled posts.

- **Development Server**: Run `npm start` to start the development server.
- **Build**: Run `npm run build` to create a production build.

### Backend

The backend is a Rust application that handles scheduling and publishing posts on LinkedIn. It communicates with the LinkedIn API and MongoDB.

- **Development Server**: Run `cargo run --bin web_server` to start the backend server.
- **Scheduler**: The scheduler is responsible for checking pending posts and publishing them at the scheduled time. It can be run using `cargo run --bin scheduler`.

### Logging

The backend uses `env_logger` for logging. Logs provide insights into the server's operation, including API requests, errors, and scheduled tasks.

## Contributing

Contributions are welcome! Please fork this repository and submit a pull request.

## License

This project is licensed under the MIT License.

## Acknowledgements

- [LinkedIn API](https://docs.microsoft.com/en-us/linkedin/)
- [Rust Programming Language](https://www.rust-lang.org/)
- [React](https://reactjs.org/)
