using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;

namespace Day4
{
    class Program
    {
        static void Main(string[] args)
        {
            var input = args[0];
            var reader = new StreamReader(input);
            List<GuardEvent> events = SleepLogParser.Parse(reader);
            events.Sort(GuardEvent.CompareByTimeStamps);
            List<Guard> guards = ShiftScheduler.ScheduleSleepTimes(events);
            var sleepiestGuard = guards.Aggregate((sleepiest, next) => next.MinutesAsleep > sleepiest.MinutesAsleep ? next : sleepiest);

            Console.WriteLine($"Sleepiest guard: {sleepiestGuard.Number}");
            var mostCommonMinute = sleepiestGuard.MostCommonMinuteAsleep();
            Console.WriteLine($"Most commonly asleep minute {mostCommonMinute}");
            Console.WriteLine($"Multiplied: {sleepiestGuard.Number * mostCommonMinute}");
        }
    }
}
