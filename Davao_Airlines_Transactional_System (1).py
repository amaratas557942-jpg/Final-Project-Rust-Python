flights = {
    "PR101": {"from": "Manila", "to": "Davao", "seats": 5, "price": 5200},
    "PR202": {"from": "Manila", "to": "Cebu", "seats": 3, "price": 3800},
    "PR305": {"from": "Cebu", "to": "Iloilo", "seats": 4, "price": 2500},
    "PR410": {"from": "Davao", "to": "Manila", "seats": 6, "price": 5400},
    "PR512": {"from": "Manila", "to": "Puerto Princesa", "seats": 4, "price": 4200},
}


class Flight:
    def __init__(self, code, from_city, to_city, seats, price):
        self.code = code
        self.from_city = from_city
        self.to_city = to_city
        self.seats = seats
        self.price = price

    def is_available(self, requested_seats=1):
        return self.seats >= requested_seats

    def book(self, seats):
        self.seats -= seats

    def __str__(self):
        return f"  {self.code:<10} {self.from_city:<15} {self.to_city:<20} {self.seats:<8} ₱ {self.price:>8,}"


class Booking:
    def __init__(self, booking_id, passenger, flight, seats):
        self.booking_id = booking_id
        self.passenger = passenger
        self.flight = flight
        self.seats = seats
        self.total = seats * flight.price

    def __str__(self):
        route = f"{self.flight.from_city} -> {self.flight.to_city}"
        return f"  {self.booking_id:<10} {self.passenger:<20} {route:<25} ₱ {self.total:>10,}"


class AirlineSystem:
    def __init__(self, flights_data):
        self.flights = {
            code: Flight(code, f["from"], f["to"], f["seats"], f["price"])
            for code, f in flights_data.items()
        }
        self.bookings = []
        self.booking_counter = 1

    def show_flights(self):
        print("\n")
        print("  ╔════════════════════════════════════════════════════════════╗")
        print("  ║           DAVAO AIRLINES FLIGHT SCHEDULE                   ║")
        print("  ╚════════════════════════════════════════════════════════════╝")
        print(f"\n  {'─'*70}")
        print(f"  {'Flight':<10} {'From':<15} {'To':<20} {'Seats':<8} {'Price':>10}")
        print(f"  {'─'*70}")
        for flight in self.flights.values():
            print(flight)
        print(f"  {'─'*70}")

    def book_flight(self):
        self.show_flights()

        code = input("\nEnter Flight Code: ").upper()
        if code not in self.flights:
            print("\nInvalid flight code.")
            return

        flight = self.flights[code]
        if not flight.is_available():
            print("\nSorry, this flight is full.")
            return

        passenger = input("Passenger Name: ")
        seats = int(input("Seats to book: "))

        if not flight.is_available(seats):
            print("\nNot enough seats available.")
            return

        booking_id = f"B{self.booking_counter:03}"
        booking = Booking(booking_id, passenger, flight, seats)
        self.bookings.append(booking)

        flight.book(seats)
        self.booking_counter += 1

        print("\nBOOKING SUCCESSFUL")
        print(f"  Booking ID : {booking.booking_id}")
        print(f"  Passenger  : {booking.passenger}")
        print(f"  Route      : {flight.from_city} -> {flight.to_city}")
        print(f"  Total Cost : ₱ {booking.total:,}")

    def view_bookings(self):
        if not self.bookings:
            print("\nNo bookings found.")
            return

        print("\n")
        print("  ╔════════════════════════════════════════════════════════════╗")
        print("  ║                DAVAO AIRLINES BOOKINGS                     ║")
        print("  ╚════════════════════════════════════════════════════════════╝")
        print(f"\n  {'─'*70}")
        print(f"  {'ID':<10} {'Passenger':<20} {'Route':<25} {'Price':>12}")
        print(f"  {'─'*70}")
        for booking in self.bookings:
            print(booking)
        print(f"  {'─'*70}")

    def run(self):
        while True:
            print("\n")
            print("  ╔══════════════════════════════╗")
            print("  ║     DAVAO AIRLINES SYSTEM    ║")
            print("  ╚══════════════════════════════╝")
            print("   1. View Flights")
            print("   2. Book Flight")
            print("   3. View Bookings")
            print("   4. Exit")

            choice = input("\nSelect option: ")

            if choice == "1":
                self.show_flights()
            elif choice == "2":
                self.book_flight()
            elif choice == "3":
                self.view_bookings()
            elif choice == "4":
                print("\nThank you for using Davao Airlines System")
                break
            else:
                print("\nInvalid option.")


system = AirlineSystem(flights)
system.run()