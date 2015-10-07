using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;

namespace cb.Utils
{
    public sealed class SemVersion : IComparable
    {
        private readonly Tuple<int, int, int, int, int, int> _tuple;
        private readonly string _version;
        private readonly string _prerelease;

        /// <summary>
        /// http://semver.org/
        /// supported version formats: 
        /// 1.0.1
        /// 1.0.2-beta
        /// 2.3.5-rc.1
        /// 4.3.2-alpha.1.2 - prerelase is two digits MAX!
        /// </summary>
        /// <param name="version"></param>
        /// <param name="prerelease"></param>
        public SemVersion(string version, string prerelease)
        {
            _version = version;
            _prerelease = prerelease;

            var list = version?.Split(".").ToList() ?? new List<string>();
            list.AddRange(new[] {"0", "0", "0"});
            version = string.Join(".", list.Take(3));

            list = prerelease?.Split(".").ToList() ?? new List<string>();
            list.AddRange(new[] { "0", "0", "0" });
            switch (list[0])
            {
                case "alpha":
                {
                    list[0] = "-3";
                    break;
                }
                case "beta":
                {
                    list[0] = "-2";
                    break;
                }
                case "rc":
                {
                    list[0] = "-1";
                    break;
                }
            }

            prerelease = string.Join(".", list.Take(3));
            version = version + "." + prerelease;

            var items = version.Split(".");
            try
            {
                _tuple = new Tuple<int, int, int, int, int, int>(
                    int.Parse(items[0]),
                    int.Parse(items[1]),
                    int.Parse(items[2]),
                    int.Parse(items[3]),
                    int.Parse(items[4]),
                    int.Parse(items[5]));
            }
            catch (Exception ex)
            {
                Debug.WriteLine("");
            }
        }

        public int Major => _tuple.Item1;
        public int Minor => _tuple.Item2;
        public int Patch => _tuple.Item3;
        public int PreRelease => _tuple.Item4;
        public int PreReleaseMajor => _tuple.Item5;
        public int PreReleaseMinor => _tuple.Item6;

        public int CompareTo(object obj)
        {
            var other = (SemVersion) obj;
            return ((IComparable)_tuple).CompareTo(other._tuple);
        }

        public override int GetHashCode()
        {
            return _tuple.GetHashCode();
        }

        public override bool Equals(object obj)
        {
            return _tuple.Equals(obj);
        }

        public override string ToString()
        {
            var sb = new StringBuilder();
            if (!_version.IsEmpty())
            {
                sb.Append(_version);
                if (!_prerelease.IsEmpty())
                {
                    sb.Append("-").Append(_prerelease);
                }
            }
            return sb.ToString();
        }
    }
}