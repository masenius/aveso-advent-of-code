using System;
using System.IO;
using System.Text.RegularExpressions;
using System.Collections.Generic;

namespace Day4 {
    public static class SleepLogParser {
        private const string TIMESTAMP_REGEX = @"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\]";
        private const string GUARD_NO_REGEX = @"Guard #(\d+)";

        public static List<GuardEvent> Parse(TextReader reader)
        {
            var events = new List<GuardEvent>();

            string line;
            while ((line = reader.ReadLine()) != null)
            {
                GuardEvent guardEvent;
                guardEvent.Time = ParseTimeStamp(line);
                if (line.EndsWith("begins shift"))
                {
                    guardEvent.Type = EventType.StartShift;
                    var match = Regex.Match(line, GUARD_NO_REGEX, RegexOptions.Compiled);
                    guardEvent.Guard = Convert.ToUInt32(match.Groups[1].Value);
                }
                else if (line.EndsWith("falls asleep"))
                {
                    guardEvent.Type = EventType.FallAsleep;
                    guardEvent.Guard = null;
                }
                else if (line.EndsWith("wakes up"))
                {
                    guardEvent.Type = EventType.WakeUp;
                    guardEvent.Guard = null;
                }
                else {
                    continue;
                }
                events.Add(guardEvent);
            }
            return events;
        }

        private static TimeStamp ParseTimeStamp(string line)
        {
            var match = Regex.Match(line, TIMESTAMP_REGEX, RegexOptions.Compiled);
            var groups = match.Groups;
            TimeStamp timeStamp;
            timeStamp.Year = Convert.ToUInt32(groups[1].Value);
            timeStamp.Month = Convert.ToUInt32(groups[2].Value);
            timeStamp.Day = Convert.ToUInt32(groups[3].Value);
            timeStamp.Hour = Convert.ToUInt32(groups[4].Value);
            timeStamp.Minute = Convert.ToUInt32(groups[5].Value);
            return timeStamp;
        }
    }
}
