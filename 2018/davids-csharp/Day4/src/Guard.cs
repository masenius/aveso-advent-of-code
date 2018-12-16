using System;
using System.Collections.Generic;
using System.Linq;

namespace Day4
{
    public struct NapTime
    {
        public TimeStamp Start;
        public uint Duration;
    }

    public class Guard
    {
        public uint Number { get; set; }
        public List<NapTime> NapTimes { get; set; }
        public uint MinutesAsleep
        {
            get {
                return (uint)this.NapTimes.Sum(n => n.Duration);
            }
        }

        public Guard() {}

        public Guard(uint number)
        {
            this.Number = number;
            this.NapTimes = new List<NapTime>();
        }

        public (uint, uint) MostCommonMinuteAsleep()
        {
            var minutes = new uint[60];
            foreach (var nap in this.NapTimes)
            {
                // Only minutes matter according to the problem description
                var timeSpan = new Span<uint>(minutes, (int)nap.Start.Minute, (int)nap.Duration);
                for (var i = 0; i < timeSpan.Length; i += 1)
                {
                    timeSpan[i] += 1;
                }
            }
            var mostCommonMinute = 0u;
            var max = 0u;
            for (var i = 0u; i < minutes.Length; i += 1)
            {
                if (minutes[i] > max)
                {
                    max = minutes[i];
                    mostCommonMinute = i;
                }
            }
            return (mostCommonMinute, max);
        }
    }
}
