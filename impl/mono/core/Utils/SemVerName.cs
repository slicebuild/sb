using System;
using System.Text;

namespace sb.Core.Utils
{
    public sealed class SemVerName : IComparable<SemVerName>
    {
        public SemVerName(
            string label,
            string name, 
            Tuple<int, int, int, int, int, int> nameVersion,
            Tuple<int, int, int, int, int, int> parentVersion)
        {
            Label = label;
            Name = name.ToLower();
            NameVersion = nameVersion;
            ParentVersion = parentVersion ?? new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0);

            var sb = new StringBuilder();
            sb.Append(Name).Append(':');
            sb.Append(NameVersion.Item1).Append('.');
            sb.Append(NameVersion.Item2).Append('.');
            sb.Append(NameVersion.Item3).Append(':');
            sb.Append(NameVersion.Item4).Append('.');
            sb.Append(NameVersion.Item5).Append('.');
            sb.Append(NameVersion.Item6).Append(':');
            sb.Append(ParentVersion.Item1).Append('.');
            sb.Append(ParentVersion.Item2).Append('.');
            sb.Append(ParentVersion.Item3).Append(':');
            sb.Append(ParentVersion.Item4).Append('.');
            sb.Append(ParentVersion.Item5).Append('.');
            sb.Append(ParentVersion.Item6);
            Value = sb.ToString();
        }

        public string Label { get; }
        public string Name { get; }
        public Tuple<int, int, int, int, int, int> NameVersion { get; }
        public Tuple<int, int, int, int, int, int> ParentVersion { get; }
       
        public int Major => NameVersion.Item1;
        public string Value { get; }

        public override bool Equals(object obj)
        {
            var other = (SemVerName) obj;
            return Value.Equals(other.Value);
        }

        public override int GetHashCode()
        {
            return Value.GetHashCode();
        }

        public int CompareTo(SemVerName other)
        {
            var t1 = new Tuple<Tuple<int, int, int, int, int, int>, Tuple<int, int, int, int, int, int>>(ParentVersion, NameVersion);
            var t2 = new Tuple<Tuple<int, int, int, int, int, int>, Tuple<int, int, int, int, int, int>>(other.ParentVersion, other.NameVersion);

            return ((IComparable) t1).CompareTo(t2);
        }

        public override string ToString()
        {
            return Label;
        }
    }
}