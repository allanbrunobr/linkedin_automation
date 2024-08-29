


# LinkedIn Automation

This project automates the process of scheduling and publishing posts on LinkedIn. It includes both a frontend (built with React) and a backend (built with Rust). The backend handles post scheduling and publishing using LinkedIn's API, while the frontend allows users to schedule posts through a simple interface.

## Features

- **Post Scheduling**: Schedule LinkedIn posts to be published at a specified date and time.
- **Post Management**: View, edit, and delete scheduled posts.
- **LinkedIn Integration**: Seamlessly publish posts on LinkedIn using their API.
- **User-Friendly Interface**: An intuitive React frontend for managing posts.
-  **Timezone Support**: Automatically handles timezone differences using a configurable offset.
## Project Structure

The project is organized into two main directories:

- `frontend/`: Contains the React application.
- `backend/`: Contains the Rust application that handles scheduling, post management, and communication with LinkedIn's API.


## Documentation
The backend documentation can be found at: [https://allanbrunobr.github.io/linkedin_automation/](https://allanbrunobr.github.io/linkedin_automation/)
This documentation provides detailed information about the backend's structure, functions, and modules.

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
   ```

   This will start the Rust web server on `http://localhost:8080`.

### Configuration

This project uses a `config.toml` file for storing configuration settings. The file should be placed in the root directory of the `backend` and include the following parameters:

```toml
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
**Important:** The `TIMEZONE_OFFSET` variable is crucial for correct date and time handling in the application. If not set, the application will not start, ensuring that all time-related operations are performed correctly.

### Obtaining a LinkedIn Access Token

To get an `access_token` from LinkedIn, you need to follow these steps:

1. Register your application with LinkedIn to get a `client_id` and `client_secret`. You can do this through the [LinkedIn Developer Portal](https://www.linkedin.com/developers/apps).

2. Implement the OAuth 2.0 authorization flow to get an authorization code and then exchange it for an `access_token`.

   For a detailed guide, refer to LinkedIn's [OAuth 2.0 documentation](https://docs.microsoft.com/en-us/linkedin/shared/authentication/authentication?context=linkedin%2Fcontext).

3. Use the obtained `access_token` to configure your `config.toml` file.

**Important:** Keep your `config.toml` file secure and do not share it publicly, as it contains sensitive information.


## Running the Application

You can run the application either using Docker Compose, which simplifies the setup process, or manually by following the traditional method. Below are instructions for both approaches.

### Option 1: Running with Docker Compose

This method uses Docker Compose to run the entire application stack, including the frontend, backend, and MongoDB, with a single command.

1. **Clone the repository:**
     
		> git clone https://github.com/yourusername/linkedin_automation.git
		> cd linkedin_automation


2. **Ensure the `config.toml` file is correctly configured:**
Make sure you have your `config.toml` file set up in the `backend` directory with the necessary configuration parameters:

	    redirect_uri = "http://localhost:3000/callback"
		access_token = "YOUR_ACCESS_TOKEN"

3. **Create a `.env` file in the `backend` directory:**

	Set the `TIMEZONE_OFFSET` variable:

		TIMEZONE_OFFSET =-3

4. **Start the application using Docker Compose:**

Navigate to the root of your project directory where the `docker-compose.yml` file is located, and run:

    docker-compose up --build

-   This command will:
    
  

>     -   Build the Docker images for the backend and frontend services.
>     -   Start the MongoDB service.
>     -   Launch the backend (including both the web server and scheduler) and frontend services.
    
    
   ### Option 2: Running Manually

If you prefer not to use Docker, you can run the application manually by setting up the services individually.
-   **Access the application:**: Open your browser and go to `http://localhost:3000` to access the application.

1.  **Clone the repository:**
The backend consists of two main binaries:

1.  **Web Server (`web_server`)**:
    -   Handles API requests for scheduling, querying, updating, and deleting posts.
    -   Start it with:
        
        
        `cargo run --bin web_server`
        
    -   The server will be available at `http://localhost:8080`.
2.  **Scheduler (`scheduler`)**:
    -   Runs in the background, checking for scheduled posts and publishing them at the appropriate time.
    -   Start it with:
        
        `cargo run --bin scheduler`
        

To run the full application:

-   Start the frontend: In the `frontend` directory, run `npm start`
-   Start the web server: In the `backend` directory, run `cargo run --bin web_server`
-   Start the scheduler: In another terminal, in the `backend` directory, run `cargo run --bin scheduler`

## API Endpoints

The backend exposes several API endpoints:

-   **POST /schedule**: Schedule a new post.
-   **GET /posts**: Retrieve all scheduled posts.
-   **GET /posts?start_date=YYYY-MM-DD&end_date=YYYY-MM-DD**: Retrieve posts scheduled within a date range.
-   **PUT /posts/**
    
    : Update an existing post by its ID.
-   **DELETE /posts/**
    
    : Delete a scheduled post.

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
                    bsonType: "long",
                    description: "Scheduled time for the post in milisseconds"
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

-   **Web Server**: Run `cargo run --bin web_server` to start the backend server.
-   **Scheduler**: Run `cargo run --bin scheduler` to start the background scheduler process.
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


