# OAuth2 Keycloak Demo - Clean

This project demonstrates a full OAuth2 integration with Keycloak, featuring user login, authenticated file upload, and backend validation.

---

## Table of Contents

- [Project Overview](#project-overview)  
- [Prerequisites](#prerequisites)  
- [Step 1: Setup Keycloak](#step-1-setup-keycloak)  
- [Step 2: Clone and Setup Project](#step-2-clone-and-setup-project)  
- [Step 3: Run Backend](#step-3-run-backend)  
- [Step 4: Run Frontend](#step-4-run-frontend)  
- [How It Works](#how-it-works)  
- [Project Structure](#project-structure)  

---

## Project Overview

This project provides a secure file upload system with OAuth2 authentication via Keycloak. The frontend runs on **http://localhost:3000**, backend on **http://localhost:8000**, and Keycloak Admin Console on **http://localhost:8080**.

---

## Prerequisites

- Docker & Docker Compose (recommended for Keycloak)  
- Rust & Cargo (for backend)  
- Node.js & npm (for frontend)  

---

## Step 1: Setup Keycloak

### Access Keycloak Admin Console

- Open your browser at: [http://localhost:8080](http://localhost:8080)  
- Log in with your admin credentials.

### 1.1 Create a Realm

- In the sidebar, click **Realms**  
- Click **Add Realm**  
- Enter **Realm Name:** `myrealm`  
- Click **Create**

### 1.2 Create Clients

#### Client 1: frontend-client

- Go to **Clients > Create**  
- Client ID: `frontend-client`  
- Protocol: `openid-connect`  
- Root URL: `http://localhost:3000`  
- Access Type: `public` (or `confidential` if needed)  
- Valid Redirect URIs: `http://localhost:3000/*`  
- Click **Save**

#### Client 2: backend-service

- Go to **Clients > Create**  
- Client ID: `backend-service`  
- Protocol: `openid-connect`  
- Access Type: `confidential`  
- Enable **Service Accounts Enabled** toggle  
- Click **Save**

### 1.3 Create Users and Roles

- Go to the **Users** tab in `myrealm`  
- Click **Add User** to create test users  
- Assign roles under the **Role Mappings** tab as needed for access control  

### 1.4 Configure Token Mappers

- Map roles/claims into tokens for backend validation as per your needs  

---

## Step 2: Clone and Setup Project

```bash
git clone https://github.com/nafisatou/authenticated-user.git
cd authenticated-user
```

---

## Step 3: Run Backend

Make sure Rust is installed.

```bash
cargo build
cargo run
```

- Backend listens on: `http://localhost:8000`  
- Accepts authenticated file uploads, validates tokens from Keycloak  

---

## Step 4: Run Frontend

In the frontend directory:

```bash
cd frontend
npm install
npm start
```

- Frontend runs on: `http://localhost:3000`  
- Automatically redirects to Keycloak login for authentication  
- After login, redirects back to frontend  

---

## How It Works

1. User accesses the frontend at `http://localhost:3000`.  
2. If not authenticated, the frontend redirects the user to Keycloak login page (`http://localhost:8080`).  
3. User logs in via Keycloak with credentials created in your realm.  
4. Keycloak redirects back to frontend with authentication token.  
5. Frontend uses token to allow file uploads, sending the token with requests to the backend.  
6. Backend verifies the token with Keycloak and stores uploaded files in the `uploads` directory.  

---

## Project Structure

- `frontend/` — React or static frontend running on port 3000  
- `backend/` — Rust Actix-web backend on port 8000  
- `uploads/` — Directory where uploaded files are saved  
- `Dockerfile` and `docker-compose.yml` — Setup files for Keycloak and app services 
