using System;
using System.Collections.Generic;

namespace Day3
{
    public class ClothPlotter
    {
        bool[,] canvas;
        List<Claim> claims;
        HashSet<(uint, uint)> overlapping;

        public ClothPlotter(bool[,] canvas)
        {
            this.canvas = canvas;
            this.claims = new List<Claim>();
            this.overlapping = new HashSet<(uint, uint)>();
        }

        public void Plot(Claim claim)
        {
            for (var x = claim.x; x < claim.x + claim.width; x++)
            {
                for (var y = claim.y; y < claim.y + claim.height; y++)
                {
                    if (this.canvas[x,y])
                    {
                        this.overlapping.Add((x, y));
                    }
                    this.canvas[x,y] = true;
                }
            }
            this.claims.Add(claim);
        }

        public int NumberOfOverlapping()
        {
            return this.overlapping.Count;
        }

        public uint? FindNonOverlappingClaim()
        {
            foreach (var claim in this.claims)
            {
                var overlaps = false;
                foreach (var point in this.overlapping)
                {
                    if (ContainsPoint(claim, point))
                    {
                        overlaps = true;
                        break;
                    }
                }
                if (!overlaps)
                {
                    return claim.id;
                }
            }
            return null;
        }

        private bool ContainsPoint(Claim claim, (uint x, uint y) point)
        {
            return (point.x >= claim.x && point.x < claim.x + claim.width
                    && point.y >= claim.y && point.y < claim.y + claim.height);
        }
    }
}
