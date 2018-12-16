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
            (uint mostCommonMinute, uint times) asleepInfo = sleepiestGuard.MostCommonMinuteAsleep();
            Console.WriteLine($"Most commonly asleep minute {asleepInfo.mostCommonMinute}");
            Console.WriteLine($"Multiplied: {sleepiestGuard.Number * asleepInfo.mostCommonMinute}");

            Guard mostRegularGuard = null;
            uint mostTimes = 0;
            uint mostCommonMinute = 0;
            foreach (var guard in guards)
            {
                // Note to self: C# doesn't support variable shadowing
                (uint mostCommonMinute, uint times) info = guard.MostCommonMinuteAsleep();
                if (info.times > mostTimes)
                {
                    mostRegularGuard = guard;
                    mostTimes = info.times;
                    mostCommonMinute = info.mostCommonMinute;
                }
            }
            Console.WriteLine($"Most regular guard: {mostRegularGuard.Number}");
            Console.WriteLine($"Most commonly asleep minute: {mostCommonMinute} ({mostTimes} times)");
            Console.WriteLine($"Multiplied: {mostRegularGuard.Number * mostCommonMinute}");
        }
    }
}
