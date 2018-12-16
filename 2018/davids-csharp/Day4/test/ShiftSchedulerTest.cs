using System;
using System.Collections.Generic;
using Xunit;

using Day4;

namespace Day4Test
{
    public class ShiftSchedulerTest
    {
        [Fact]
        public void TestScheduleSleepTimes()
        {
            var input = new List<GuardEvent> {
                new GuardEvent { Time = new TimeStamp { Year = 1518, Month = 11, Day = 1, Hour = 0, Minute = 0 }, Type = EventType.StartShift, Guard = 10 },
                new GuardEvent { Time = new TimeStamp { Year = 1518, Month = 11, Day = 1, Hour = 0, Minute = 5 }, Type = EventType.FallAsleep, Guard = null },
                new GuardEvent { Time = new TimeStamp { Year = 1518, Month = 11, Day = 1, Hour = 0, Minute = 25 }, Type = EventType.WakeUp, Guard = null },

                new GuardEvent { Time = new TimeStamp { Year = 1518, Month = 11, Day = 1, Hour = 23, Minute = 58 }, Type = EventType.StartShift, Guard = 99 },
                new GuardEvent { Time = new TimeStamp { Year = 1518, Month = 11, Day = 2, Hour = 0, Minute = 40 }, Type = EventType.FallAsleep, Guard = null },
                new GuardEvent { Time = new TimeStamp { Year = 1518, Month = 11, Day = 2, Hour = 0, Minute = 50 }, Type = EventType.WakeUp, Guard = null }
            };

            var expected = new List<Guard> {
                new Guard { Number = 10, NapTimes = new List<NapTime> {
                        new NapTime { Start = new TimeStamp { Year = 1518, Month = 11, Day = 1, Hour = 0, Minute = 5 }, Duration = 20 }}
                },
                new Guard { Number = 99, NapTimes = new List<NapTime> {
                        new NapTime { Start = new TimeStamp { Year = 1518, Month = 11, Day = 2, Hour = 0, Minute = 40 }, Duration = 10 }}
                }
            };
            var actual = ShiftScheduler.ScheduleSleepTimes(input);

            // XUnit's deep equal doesn't seem to work recursively...
            Assert.Equal(expected.Count, actual.Count);
            var i1 = expected.GetEnumerator();
            var i2 = actual.GetEnumerator();
            while (i1.MoveNext())
            {
                i2.MoveNext();
                CompareGuards(i1.Current, i2.Current);
            }
        }

        private void CompareGuards(Guard expected, Guard actual)
        {
            Assert.Equal(expected.Number, actual.Number);
            Assert.Equal(expected.NapTimes, actual.NapTimes);
        }
    }
}
