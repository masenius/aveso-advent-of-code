using System.Collections.Generic;
using System.Linq;

namespace Day4
{
    public static class ShiftScheduler
    {
        /// <summary>
        /// Requires a list of events sorted by timestamp!
        /// </summary>
        public static List<Guard> ScheduleSleepTimes(List<GuardEvent> events)
        {
            var guards = new Dictionary<uint, Guard>();

            var currentNap = new NapTime();
            Guard currentGuard = null;
            foreach (var guardEvent in events)
            {
                switch (guardEvent.Type)
                {
                    case EventType.StartShift:
                        // Shouldn't be null
                        uint guardNumber = guardEvent.Guard.Value;
                        if (!guards.ContainsKey(guardNumber))
                        {
                            guards.Add(guardNumber, new Guard(guardNumber));
                        }
                        currentGuard = guards[guardNumber];
                        break;
                    // Just assume that events always come in their expected order
                    // i.e. there is always an active guard on shift,
                    // and no one wakes up before they fall asleep
                    case EventType.FallAsleep:
                        currentNap.Start = guardEvent.Time;
                        break;
                    case EventType.WakeUp:
                        var startMinutes = currentNap.Start.Hour * 60 + currentNap.Start.Minute;
                        var endMinutes = guardEvent.Time.Hour * 60 + guardEvent.Time.Minute;
                        currentNap.Duration = endMinutes - startMinutes;
                        currentGuard.NapTimes.Add(currentNap);
                        break;
                }
            }
            return guards.Values.ToList();
        }
    }
}
