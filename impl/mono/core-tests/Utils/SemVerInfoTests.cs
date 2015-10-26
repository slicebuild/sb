using System;
using sb.Core.Utils;
using Xunit;

namespace sb.Core.Tests.Utils
{
    public class SemVerInfoTests
    {
        private readonly Tuple<int, int, int, int, int, int> _emptySemVer =
            new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0);

        [Fact]
        public void SimpleNameOnly()
        {
            var svi = new SemVerInfo("Ruby");
            Assert.Equal(svi.Label, "Ruby");
            Assert.Equal(svi.Name, "ruby");
            Assert.Equal(svi.BunchSemVer, _emptySemVer);
            Assert.Equal(svi.NameSemVer, _emptySemVer);
        }

        [Fact]
        public void WithDashesNameOnly()
        {
            var svi = new SemVerInfo("Name-one-two");
            Assert.Equal(svi.Label, "Name-one-two");
            Assert.Equal(svi.Name, "name-one-two");
            Assert.Equal(svi.BunchSemVer, _emptySemVer);
            Assert.Equal(svi.NameSemVer, _emptySemVer);
        }

        [Fact]
        public void WithDashesAndUnderscoresNameOnly()
        {
            var svi = new SemVerInfo("_Name-one_two");
            Assert.Equal(svi.Label, "_Name-one_two");
            Assert.Equal(svi.Name, "_name-one_two");
            Assert.Equal(svi.BunchSemVer, _emptySemVer);
            Assert.Equal(svi.NameSemVer, _emptySemVer);
        }

        [Fact]
        public void OneDigit()
        {
            var svi = new SemVerInfo("name-2");
            Assert.Equal(svi.Label, "name-2");
            Assert.Equal(svi.Name, "name");
            Assert.Equal(svi.BunchSemVer, _emptySemVer);
            Assert.Equal(svi.NameSemVer, new Tuple<int, int, int, int, int, int>(2, 0, 0, 0, 0, 0));
        }

        [Fact]
        public void TwoDigits()
        {
            var svi = new SemVerInfo("name-21");
            Assert.Equal(svi.Label, "name-21");
            Assert.Equal(svi.Name, "name");
            Assert.Equal(svi.BunchSemVer, _emptySemVer);
            Assert.Equal(svi.NameSemVer, new Tuple<int, int, int, int, int, int>(21, 0, 0, 0, 0, 0));
        }

        [Fact]
        public void TwoPlaces()
        {
            var svi = new SemVerInfo("Name-2.1");
            Assert.Equal(svi.Label, "Name-2.1");
            Assert.Equal(svi.Name, "name");
            Assert.Equal(svi.BunchSemVer, _emptySemVer);
            Assert.Equal(svi.NameSemVer, new Tuple<int, int, int, int, int, int>(2, 1, 0, 0, 0, 0));
        }

        [Fact]
        public void TwoDigitsTwoPlaces()
        {
            var svi = new SemVerInfo("name-21.11");
            Assert.Equal(svi.Label, "name-21.11");
            Assert.Equal(svi.Name, "name");
            Assert.Equal(svi.BunchSemVer, _emptySemVer);
            Assert.Equal(svi.NameSemVer, new Tuple<int, int, int, int, int, int>(21, 11, 0, 0, 0, 0));
        }

        [Fact]
        public void ThreePlaces()
        {
            var svi = new SemVerInfo("name-2.1.1");
            Assert.Equal(svi.Label, "name-2.1.1");
            Assert.Equal(svi.Name, "name");
            Assert.Equal(svi.BunchSemVer, _emptySemVer);
            Assert.Equal(svi.NameSemVer, new Tuple<int, int, int, int, int, int>(2, 1, 1, 0, 0, 0));
        }

        [Fact]
        public void TwoDigitsThreePlaces()
        {
            var svi = new SemVerInfo("name-21.2.234");
            Assert.Equal(svi.Label, "name-21.2.234");
            Assert.Equal(svi.Name, "name");
            Assert.Equal(svi.BunchSemVer, _emptySemVer);
            Assert.Equal(svi.NameSemVer, new Tuple<int, int, int, int, int, int>(21, 2, 234, 0, 0, 0));
        }

        [Fact]
        public void PreReleaseAlpha()
        {
            var svi = new SemVerInfo("name-21.2.234-alpha");
            Assert.Equal(svi.Label, "name-21.2.234-alpha");
            Assert.Equal(svi.Name, "name");
            Assert.Equal(svi.BunchSemVer, _emptySemVer);
            Assert.Equal(svi.NameSemVer, new Tuple<int, int, int, int, int, int>(21, 2, 234, -3, 0, 0));
        }

        [Fact]
        public void PreReleaseBeta()
        {
            var svi = new SemVerInfo("name-21.2.234-beta.1");
            Assert.Equal(svi.Label, "name-21.2.234-beta.1");
            Assert.Equal(svi.Name, "name");
            Assert.Equal(svi.BunchSemVer, _emptySemVer);
            Assert.Equal(svi.NameSemVer, new Tuple<int, int, int, int, int, int>(21, 2, 234, -2, 1, 0));
        }

        [Fact]
        public void PreReleaseRc()
        {
            var svi = new SemVerInfo("name-21.2.234-rc.11");
            Assert.Equal(svi.Label, "name-21.2.234-rc.11");
            Assert.Equal(svi.Name, "name");
            Assert.Equal(svi.BunchSemVer, _emptySemVer);
            Assert.Equal(svi.NameSemVer, new Tuple<int, int, int, int, int, int>(21, 2, 234, -1, 11, 0));
        }

        [Fact]
        public void WithBunchSemVer()
        {
            var sviB = new SemVerInfo("parent-1.2.3-rc.4");
            var svi = new SemVerInfo(sviB.NameSemVer, "name-21.2.234-beta.1");
            Assert.Equal(svi.Label, "name-21.2.234-beta.1");
            Assert.Equal(svi.Name, "name");
            Assert.Equal(svi.BunchSemVer, new Tuple<int, int, int, int, int, int>(1, 2, 3, -1, 4, 0));
            Assert.Equal(svi.NameSemVer, new Tuple<int, int, int, int, int, int>(21, 2, 234, -2, 1, 0));
        }

        [Fact]
        public void ValueEqual()
        {
            var sviB1 = new SemVerInfo("parent-1.2.3-rc.4");
            var sviB2 = new SemVerInfo("parent-1.2.3-rc.4");
            var svi1 = new SemVerInfo(sviB1.NameSemVer, "name-21.2.234-beta.1");
            var svi2 = new SemVerInfo(sviB2.NameSemVer, "name-21.2.234-beta.1");
            Assert.Equal(svi1.Value, svi2.Value);
        }

        [Fact]
        public void ValueComparesEqual()
        {
            var sviB1 = new SemVerInfo("parent-1.2.3-rc.4");
            var sviB2 = new SemVerInfo("parent-1.2.3-rc.4");
            var svi1 = new SemVerInfo(sviB1.NameSemVer, "name-21.2.234-beta.1");
            var svi2 = new SemVerInfo(sviB2.NameSemVer, "name-21.2.234-beta.1");

            var c1 = svi1.CompareTo(svi2);
            Assert.Equal(c1, 0);
        }

        [Fact]
        public void ValueComparesLowerByBunchSemVer()
        {
            var sviB1 = new SemVerInfo("parent-1.2.3-rc.3");
            var sviB2 = new SemVerInfo("parent-1.2.3-rc.4");
            var svi1 = new SemVerInfo(sviB1.NameSemVer, "name-21.2.234-beta.1");
            var svi2 = new SemVerInfo(sviB2.NameSemVer, "name-21.2.234-beta.1");

            var c1 = svi1.CompareTo(svi2);
            Assert.Equal(c1, -1);
        }

        [Fact]
        public void ValueComparesLowerByNameSemVer()
        {
            var sviB1 = new SemVerInfo("parent-1.2.3-rc.4");
            var sviB2 = new SemVerInfo("parent-1.2.3-rc.4");
            var svi1 = new SemVerInfo(sviB1.NameSemVer, "name-21.2.233-beta.1");
            var svi2 = new SemVerInfo(sviB2.NameSemVer, "name-21.2.234-beta.1");

            var c1 = svi1.CompareTo(svi2);
            Assert.Equal(c1, -1);
        }

        [Fact]
        public void ValueComparesBiggerByNameSemVer()
        {
            var sviB1 = new SemVerInfo("parent-1.2.3-rc.4");
            var sviB2 = new SemVerInfo("parent-1.2.3-rc.4");
            var svi1 = new SemVerInfo(sviB1.NameSemVer, "name-21.2.234");
            var svi2 = new SemVerInfo(sviB2.NameSemVer, "name-21.2.234-beta.1");

            var c1 = svi1.CompareTo(svi2);
            Assert.Equal(c1, 1);
        }

        [Fact]
        public void ValueComparesBiggerByName()
        {
            var sviB1 = new SemVerInfo("parent-1.2.3-rc.4");
            var sviB2 = new SemVerInfo("parent-1.2.3-rc.4");
            var svi1 = new SemVerInfo(sviB1.NameSemVer, "name-21.2.234");
            var svi2 = new SemVerInfo(sviB2.NameSemVer, "lame-21.2.234");

            var c1 = svi1.CompareTo(svi2);
            Assert.Equal(c1, 1);
        }

        [Fact]
        public void ToStringEqualsLabel()
        {
            var sviB = new SemVerInfo("parent-1.2.3-rc.4");
            var svi = new SemVerInfo(sviB.NameSemVer, "Name-21.2.234-beta.1");
            Assert.Equal(svi.Label, "Name-21.2.234-beta.1");
            Assert.Equal(svi.ToString(), "Name-21.2.234-beta.1");
        }
    }
}