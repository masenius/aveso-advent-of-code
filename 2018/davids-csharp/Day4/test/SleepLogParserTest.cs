using System;
using System.IO;
using System.Collections.Generic;
using Xunit;

using Day4;

namespace Day4Test
{
    public class SleepLogParserTest
    {
        [Fact]
        public void TestParse()
        {
            var input = @"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up";
            var reader = new StringReader(input);

            List<GuardEvent> expected = new List<GuardEvent> {
                new GuardEvent { Time = new TimeStamp { Year = 1518, Month = 11, Day = 1, Hour = 0, Minute = 0 }, Type = EventType.StartShift, Guard = 10 },
                new GuardEvent { Time = new TimeStamp { Year = 1518, Month = 11, Day = 1, Hour = 0, Minute = 5 }, Type = EventType.FallAsleep, Guard = null },
                new GuardEvent { Time = new TimeStamp { Year = 1518, Month = 11, Day = 1, Hour = 0, Minute = 25 }, Type = EventType.WakeUp, Guard = null }
            };
            var actual = SleepLogParser.Parse(reader);

            Assert.Equal(expected, actual);
        }
    }
}
