using System;
using System.Diagnostics;
using System.Text;

namespace sb.Core.Utils
{
    public sealed class SemVerInfo : IComparable<SemVerInfo>
    {
        public SemVerInfo(string label)
            : this(new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0), label)
        {
        }
        
        public SemVerInfo(Tuple<int, int, int, int, int, int> bunchSemVer, string label)
        {
            Tuple<int, int, int, int, int, int> semver;
            string name;
            Parse(label, out semver, out name);

            Label = label;

            BunchSemVer = bunchSemVer;
            NameSemVer = semver;
            Name = name;

//            var sb = new StringBuilder();
//            sb.Append(BunchSemVer.Item1).Append('.');
//            sb.Append(BunchSemVer.Item2).Append('.');
//            sb.Append(BunchSemVer.Item3).Append(':');
//            sb.Append(BunchSemVer.Item4).Append('.');
//            sb.Append(BunchSemVer.Item5).Append('.');
//            sb.Append(BunchSemVer.Item6).Append(':');
//            
//            sb.Append(NameSemVer.Item1).Append('.');
//            sb.Append(NameSemVer.Item2).Append('.');
//            sb.Append(NameSemVer.Item3).Append(':');
//            sb.Append(NameSemVer.Item4).Append('.');
//            sb.Append(NameSemVer.Item5).Append('.');
//            sb.Append(NameSemVer.Item6).Append(':');
//
//            sb.Append(Name).Append(':');
            Value = new Tuple<Tuple<int, int, int, int, int, int>, Tuple<int, int, int, int, int, int>, string>(BunchSemVer, NameSemVer, Name);
        }

        /// <summary>
        /// http://semver.org/
        /// supported version formats: 
        /// 1.0.1
        /// 1.0.2-beta
        /// 2.3.5-rc.1
        /// 4.3.2-alpha.1.2 - prerelase is TWO digits MAX!
        /// </summary>
        private static void Parse(string label, out Tuple<int, int, int, int, int, int> semver, out string name)
        {
            semver = null;
            name = "";
            
            try
            {
                var text = label.Trim().ToLowerInvariant();
                var sb = new StringBuilder();
                var dash = false;
                var namesDone = false;
                var versions = new[] { 0, 0, 0, 0, 0, 0 };
                var versionIndex = 0;

                for (var i = 0; i < text.Length; i++)
                {
                    var ch = text[i];

                    if (ch == ' ')
                        continue;

                    if (dash && !char.IsDigit(ch))
                    {
                        sb.Append('-');
                        dash = false;
                    }

                    if (ch == '-' && !namesDone)
                    {
                        dash = true;
                        continue;
                    }

                    if (dash && char.IsDigit(ch))
                    {
                        name = sb.ToString();
                        namesDone = true;
                        dash = false;
                        sb = new StringBuilder();
                    }

                    if (ch == '.')
                        namesDone = true;

                    if (ch == '.' || ch == '-')
                    {
                        var verPart = ConvertVersion(sb.ToString());
                        versions[versionIndex] = verPart;
                        versionIndex++;
                        sb = new StringBuilder();
                        continue;
                    }

                    sb.Append(ch);
                }

                if (sb.Length > 0)
                {
                    var s = sb.ToString();
                    if (!namesDone)
                        name = s;
                    else
                        versions[versionIndex] = ConvertVersion(s);
                }

                semver = new Tuple<int, int, int, int, int, int>(versions[0], versions[1], versions[2], versions[3], versions[4], versions[5]);
            }
            catch (Exception ex)
            {
                Trace.WriteLine(ex);
                throw;
            }
        }

        private static int ConvertVersion(string txt)
        {
            switch (txt)
            {
                case "alpha":
                    return -3;
                case "beta":
                    return -2;
                case "rc":
                    return -1;
            }
            return int.Parse(txt);
        }

        public string Label { get; }
        public Tuple<int, int, int, int, int, int> BunchSemVer { get; }
        public Tuple<int, int, int, int, int, int> NameSemVer { get; }
        public string Name { get; }
        public Tuple<Tuple<int, int, int, int, int, int>, Tuple<int, int, int, int, int, int>, string> Value { get; }

        public int NameMajor => NameSemVer.Item1;

        public override bool Equals(object obj)
        {
            if (!(obj is SemVerInfo))
                throw new ArgumentException();
            var other = (SemVerInfo) obj;
            return Value.Equals(other.Value);
        }

        public override int GetHashCode()
        {
            return Value.GetHashCode();
        }

        public int CompareTo(SemVerInfo other)
        {
            var v1 = (IComparable) Value;
            var v2 = (IComparable) other.Value;
            return v1.CompareTo(v2);
        }

        public int CompareByNameSemVer(SemVerInfo other)
        {
            var v1 = (IComparable)NameSemVer;
            var v2 = (IComparable)other.NameSemVer;
            return v1.CompareTo(v2);
        }

        public override string ToString()
        {
            return Label;
        }
    }
}