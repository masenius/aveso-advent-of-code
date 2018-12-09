using System;
using System.Text.RegularExpressions;

namespace Day3
{
    public class ClaimParser
    {
        private const string CLAIM_REGEX = @"#(?<id>\d+) @ (?<x>\d+),(?<y>\d+): (?<width>\d+)x(?<height>\d+)$";

        public static Claim Parse(string input)
        {
            var match = Regex.Match(input, CLAIM_REGEX, RegexOptions.Compiled);
            var groups = match.Groups;
            return new Claim(Convert.ToUInt32(groups["id"].Value),
                             Convert.ToUInt32(groups["x"].Value),
                             Convert.ToUInt32(groups["y"].Value),
                             Convert.ToUInt32(groups["width"].Value),
                             Convert.ToUInt32(groups["height"].Value));
        }
    }
}
