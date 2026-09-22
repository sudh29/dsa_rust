class Appointment:
    def __init__(self, start, end):
        self.start = start
        self.end = end


def findConflictingAppointments(appointments):
    appointments.sort(key=lambda x: x.start)

    conflicting_appointments = []

    for i in range(len(appointments) - 1):
        current_appointment = appointments[i]

        # Check for conflicts with future appointments
        for j in range(i + 1, len(appointments)):
            future_appointment = appointments[j]
            if current_appointment.end > future_appointment.start:
                conflicting_appointments.append(
                    (current_appointment, future_appointment)
                )

    return conflicting_appointments


# Example usage:
appointments = [
    Appointment(1, 5),
    Appointment(3, 7),
    Appointment(2, 6),
    Appointment(8, 10),
    Appointment(5, 8),
]

conflicts = findConflictingAppointments(appointments)

if conflicts:
    for conflict in conflicts:
        print(
            f"Conflicting Appointments: {conflict[0].start}-{conflict[0].end} and {conflict[1].start}-{conflict[1].end}"
        )
else:
    print("No conflicting appointments.")
