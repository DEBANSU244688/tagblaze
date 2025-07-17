
# TAGBLAZE

*Transform Support with Speed and Precision*

![last commit](https://img.shields.io/github/last-commit/DEBANSU244688/tagblaze)
![Rust](https://img.shields.io/badge/rust-100%25-orange)
![Languages](https://img.shields.io/github/languages/count/DEBANSU244688/tagblaze)

**Built with the tools and technologies:**

![Markdown](https://img.shields.io/badge/-Markdown-black?logo=markdown)
![Rust](https://img.shields.io/badge/-Rust-orange?logo=rust)
![TOML](https://img.shields.io/badge/-TOML-brown?logo=toml)

---

## Table of Contents

- [Overview](#overview)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Testing](#testing)
  - [PowerShell Flow](#powershell-flow)
  - [API Summary Table](#api-summary-table)
- [Documentation](#documentation)
- [Contribution](#contribution)
- [Tags](#tags)
- [Author](#author)
- [Star if you like it](#star-if-you-like-it)
- [License](#license)

---

## Overview

**TagBlaze** is a high-performance Rust backend tailored for efficient customer support ticket management.

It offers a scalable, modular architecture that simplifies handling **tickets**, **tags**, and **user authentication**, ensuring clean and secure support workflows.

### ✨ Why TagBlaze?

- ⚡ **High-Speed API** — Powered by **Axum + SeaORM**
- 🔐 **JWT Auth** — Secure endpoints with role-based access
- 📦 **Modular Codebase** — Clean structure for maintainability
- 🚨 **Robust Logging & Errors**
- 🧪 **Comprehensive Testing Support** — With PowerShell + REST

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/)
- PostgreSQL (running & accessible)
- [SeaORM CLI](https://www.sea-ql.org/SeaORM/docs/install-and-setup/)

### Installation

```bash
git clone https://github.com/DEBANSU244688/tagblaze
cd tagblaze/server
cargo build
```

Ensure your `.env` file is configured correctly.

---

## Usage

Run the development server:

```bash
cargo run
```

The API will be live at `http://localhost:3000`.

---

## Testing

### PowerShell Flow

```powershell
# 1️⃣ Reset DB  + Login + Setup Auth Header
# 🔄 Reset DB
Invoke-RestMethod http://localhost:3000/admin/dev/reset-db -Method Post

# 🔐 Login (default seeded user)
$loginBody = @{
  email = "zoya@tagblaze.dev"
  password = "devpass123"
} | ConvertTo-Json

$loginResponse = Invoke-RestMethod http://localhost:3000/auth/login -Method Post -Body $loginBody -ContentType "application/json"
$token = $loginResponse.token

# 🪪 Auth Header
$headers = @{
  Authorization = "Bearer $token"
}

# 2️⃣ Health Route 
Invoke-RestMethod http://localhost:3000/health

# 3️⃣  Register New Agent
$registerBody = @{
  email = "test@tagblaze.dev"
  name = "Test User"
  password = "test1234"
  role = "agent"
} | ConvertTo-Json

Invoke-RestMethod http://localhost:3000/auth/register -Method Post -Body $registerBody -ContentType "application/json"

# 4️⃣ Login as Agent
$newLoginBody = @{
  email = "test@tagblaze.dev"
  password = "test1234"
} | ConvertTo-Json

$newLoginResponse = Invoke-RestMethod http://localhost:3000/auth/login -Method Post -Body $newLoginBody -ContentType "application/json"
$newToken = $newLoginResponse.token

# (Optional) Update header to use new user token
$headers = @{ Authorization = "Bearer $newToken" }

# 5️⃣ Me
Invoke-RestMethod http://localhost:3000/auth/me -Headers $headers

# 6️⃣ Reset DB Again (Clean Slate)
Invoke-RestMethod http://localhost:3000/admin/dev/reset-db -Method Post

# 7️⃣ Login as Zoya (seeded user)
$loginBody = @{
  email = "zoya@tagblaze.dev"
  password = "devpass123"
} | ConvertTo-Json

$loginResponse = Invoke-RestMethod http://localhost:3000/auth/login -Method Post -Body $loginBody -ContentType "application/json"
$token = $loginResponse.token
$headers = @{ Authorization = "Bearer $token" }

# 8️⃣ Create Ticket
$ticket = @{
  title = "Fix Login UI"
  description = "Login form overflows on mobile"
  status = "open"
} | ConvertTo-Json

Invoke-RestMethod http://localhost:3000/tickets -Method Post -Headers $headers -Body $ticket -ContentType "application/json"

# 9️⃣ Get All Tickets
Invoke-RestMethod http://localhost:3000/tickets -Headers $headers

# 🔟 Get Ticket By ID
Invoke-RestMethod http://localhost:3000/tickets/1 -Headers $headers

# 1️⃣1️⃣ Update Ticket
$updateTicket = @{
  title = "Fix Login Modal"
  description = "Login modal not centered"
  status = "in_progress"
} | ConvertTo-Json

Invoke-RestMethod http://localhost:3000/tickets/1 -Method Put -Headers $headers -Body $updateTicket -ContentType "application/json"

# 1️⃣2️⃣ Delete Ticket
Invoke-RestMethod http://localhost:3000/tickets/1 -Method Delete -Headers $headers

# 1️⃣3️⃣ Create Tag
$tag = @{
  name = "Bug"
} | ConvertTo-Json

Invoke-RestMethod http://localhost:3000/tags/create -Method Post -Headers $headers -Body $tag -ContentType "application/json"

# 1️⃣4️⃣ Get All Tags
Invoke-RestMethod http://localhost:3000/tags

# 1️⃣5️⃣ Get Tag By ID
Invoke-RestMethod http://localhost:3000/tags/1

# 1️⃣6️⃣ Update Tag
$updateTag = @{
  name = "Critical Bug"
} | ConvertTo-Json

Invoke-RestMethod http://localhost:3000/tags/1 -Method Put -Headers $headers -Body $updateTag -ContentType "application/json"

# 1️⃣7️⃣  Delete Tag
Invoke-RestMethod http://localhost:3000/tags/1 -Method Delete

# 1️⃣8️⃣ Get Tags for a Ticket
Invoke-RestMethod http://localhost:3000/relations/2/tags

# 1️⃣9️⃣ Assign Tag To Ticket
Invoke-RestMethod http://localhost:3000/relations/2/tags/2 -Method Post -Headers $headers

# 2️⃣0️⃣ Remove Tag From Ticket
Invoke-RestMethod http://localhost:3000/relations/2/tags/2 -Method Delete

# If in db no rows found for 1️⃣8️⃣-2️⃣0️⃣, then try again with 6️⃣
```

### API Summary Table

| #   | Endpoint                              | Auth? | Method | Description                         |
|-----|----------------------------------------|-------|--------|-------------------------------------|
| 1️⃣ | `/admin/dev/reset-db`                  | ❌     | POST   | Reset DB with seeded users          |
| 2️⃣ | `/auth/register`                       | ❌     | POST   | Register a user                     |
| 3️⃣ | `/auth/login`                          | ❌     | POST   | Get JWT token                       |
| 4️⃣ | `/auth/me`                             | ✅     | GET    | Get current user                    |
| 5️⃣ | `/health`                              | ❌     | GET    | Server health                       |
| 6️⃣ | `/tickets`                             | ✅     | POST   | Create a new ticket                 |
| 7️⃣ | `/tickets`                             | ✅     | GET    | Get all tickets                     |
| 8️⃣ | `/tickets/{id}`                        | ✅     | GET    | Get ticket by ID                    |
| 9️⃣ | `/tickets/{id}`                        | ✅     | PUT    | Update ticket                       |
| 🔟 | `/tickets/{id}`                        | ✅     | DELETE | Delete ticket                       |
| 1️⃣1️⃣ | `/tags`                              | ✅     | POST   | Create tag                          |
| 1️⃣2️⃣ | `/tags`                              | ❌     | GET    | Get all tags                        |
| 1️⃣3️⃣ | `/tags/{id}`                         | ❌     | GET    | Get tag by ID                       |
| 1️⃣4️⃣ | `/tags/{id}`                         | ✅     | PUT    | Update tag                          |
| 1️⃣5️⃣ | `/tags/{id}`                         | ✅     | DELETE | Delete tag                          |
| 1️⃣6️⃣ | `/relations/{ticket_id}/tags`        | ❌     | GET    | Get tags for ticket                 |
| 1️⃣7️⃣ | `/relations/{ticket_id}/tags/{id}`   | ✅     | POST   | Assign tag to ticket                |
| 1️⃣8️⃣ | `/relations/{ticket_id}/tags/{id}`   | ✅     | DELETE | Remove tag from ticket              |

---

## 📚 Documentation

To generate and view Rust Docs:

```bash
cargo doc --no-deps --open
```

Docs are generated in:

```
target/doc/tagblaze/index.html
```

---

## 🤝 Contribution

Want to improve TagBlaze? You're welcome!

- Clone the repo
- Create a branch
- Raise a PR

Open an issue if you discover bugs or want to suggest a feature.

---

## 🏷️ Tags

`Rust` · `Axum` · `Seaorm` · `Jwt` · `Postgres` · `Backend` · `Support-System` · `Helpdesk` · `Ticketing`

---

## 👤 Author

**Debansu Debadutta Das**  
🔗 [My Portfolio](https://sites.google.com/view/debansu-debadutta-das/home)  
📫 Email: debansudebaduttadas@gmail.com

---

## ⭐ Star if you like it

If you found this project helpful, show some love by giving it a ⭐ on [GitHub](https://github.com/DEBANSU244688/tagblaze)!

---

## 🪪 License

MIT License © 2025 Debansu