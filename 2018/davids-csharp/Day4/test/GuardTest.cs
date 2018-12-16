using System.Collections.Generic;
using Xunit;

using Day4;

namespace Day4Test
{
    public class GuardTest
    {
        [Fact]
        public void TestMinutesAsleep()
        {
            var guard = new Guard { Number = 10, NapTimes = new List<NapTime> {
                    new NapTime { Start = new TimeStamp { Year = 1518, Month = 11, Day = 1, Hour = 0, Minute = 5 }, Duration = 20 },
                    new NapTime { Start = new TimeStamp { Year = 1518, Month = 11, Day = 2, Hour = 10, Minute = 24 }, Duration = 10 }}
            };

            var expected = 30u;
            var actual = guard.MinutesAsleep;

            Assert.Equal(expected, actual);
        }

        [Fact]
        public void TestMostCommonMinuteAsleep()
        {
            var guard = new Guard { Number = 10, NapTimes = new List<NapTime> {
                    new NapTime { Start = new TimeStamp { Year = 1518, Month = 11, Day = 1, Hour = 0, Minute = 5 }, Duration = 20 },
                    new NapTime { Start = new TimeStamp { Year = 1518, Month = 11, Day = 2, Hour = 10, Minute = 24 }, Duration = 10 }}
            };

            var expected = 24u;
            var actual = guard.MostCommonMinuteAsleep();

            Assert.Equal(expected, actual);
        }
    }
}
