# Davao Airlines Transactional System

**Python Console Application**

---

## Overview

Davao Airlines Transactional System is a Python console application that simulates a basic airline reservation workflow. It allows users to view available domestic Philippine flights, book seats, and review existing bookings — all through an interactive text menu.

---

## Project Info

| Property | Details |
|---|---|
| Language | Python 3 |
| File | `Davao_Airlines_Transactional_System.py` |
| Interface | Interactive terminal / console |
| Dependencies | None (standard library only) |
| Run command | `python Davao_Airlines_Transactional_System.py` |

---

## Features

- View all available flights with route, remaining seat count, and price
- Book a flight by flight code, passenger name, and number of seats
- Seat availability validation before confirming a booking
- Auto-generated booking IDs (`B001`, `B002`, …)
- View all bookings for the current session with total cost per booking
- Console UI with box-drawing borders for a structured display

---

## Pre-loaded Flights

| Code | From | To | Seats | Price (₱) |
|---|---|---|---|---|
| PR101 | Manila | Davao | 5 | 5,200 |
| PR202 | Manila | Cebu | 3 | 3,800 |
| PR305 | Cebu | Iloilo | 4 | 2,500 |
| PR410 | Davao | Manila | 6 | 5,400 |
| PR512 | Manila | Puerto Princesa | 4 | 4,200 |

---

## Class Structure

### `Flight`

Represents a single flight with its code, origin, destination, available seat count, and price per seat.

| Method | Description |
|---|---|
| `is_available(requested_seats)` | Returns `True` if enough seats remain |
| `book(seats)` | Decrements available seat count by the given number |
| `__str__()` | Formatted row string for the flight schedule table |

### `Booking`

Records a confirmed reservation: the generated booking ID, passenger name, the `Flight` object, seat count, and calculated total cost.

| Attribute | Description |
|---|---|
| `booking_id` | Auto-generated string, e.g. `B001` |
| `passenger` | Name entered by the user |
| `flight` | Reference to the `Flight` object booked |
| `seats` | Number of seats reserved |
| `total` | `seats × flight.price` |

### `AirlineSystem`

The main controller that owns the flight registry and booking list. Instantiated once and runs the menu loop.

| Method | Description |
|---|---|
| `show_flights()` | Prints the formatted flight schedule table |
| `book_flight()` | Guides the user through selecting a flight and completing a booking |
| `view_bookings()` | Prints all bookings made in the current session |
| `run()` | Starts the main menu loop; exits on option 4 |

---

## Usage

### Running the App

```bash
python Davao_Airlines_Transactional_System.py
```

### Menu Options

| Option | Action |
|---|---|
| `1` | **View Flights** — display all available routes with seats and prices |
| `2` | **Book Flight** — enter a flight code, passenger name, and seat count to reserve |
| `3` | **View Bookings** — list all bookings made during this session |
| `4` | **Exit** — quit the application |

### Booking Flow

1. Select option `2` from the main menu
2. The flight schedule is displayed automatically
3. Enter a valid flight code (e.g. `PR101`) — input is case-insensitive
4. Enter the passenger name
5. Enter the number of seats (must not exceed remaining availability)
6. A booking confirmation is printed with the ID, route, and total cost

---

## Notes & Limitations

- **No persistence** — all flights and bookings reset when the program exits.
- **No cancellation** — bookings cannot be removed once created.
- **Single-session only** — no login or multi-user support.
- Input validation is minimal — non-integer seat input will raise an unhandled exception.
- Seats are deducted immediately upon booking with no confirmation step.

---

## Possible Extensions

- Save/load flights and bookings to a JSON or CSV file for persistence
- Add a cancellation option that restores seats to the flight
- Validate seat input to handle non-numeric entries gracefully
- Add search / filter by origin or destination city
- Extend with a simple Flask or FastAPI layer for a web interface
