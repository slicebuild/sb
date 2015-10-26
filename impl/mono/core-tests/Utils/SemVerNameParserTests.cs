using System;
using sb.Core.Utils;
using Xunit;

namespace sb.Core.Tests.Utils
{
    public class SemVerNameParserTests
    {
        [Fact]
        public void SimpleNameOnly()
        {
            var svn = SemVerNameParser.Parse("Name");
            Assert.Equal(svn.Label, "Name");
            Assert.Equal(svn.Name, "name");
            Assert.Equal(svn.NameVersion, new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0));
            Assert.Equal(svn.ParentVersion, new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0));
        }

        [Fact]
        public void WithDashesNameOnly()
        {
            var svn = SemVerNameParser.Parse("Name-one-two");
            Assert.Equal(svn.Label, "Name-one-two");
            Assert.Equal(svn.Name, "name-one-two");
            Assert.Equal(svn.NameVersion, new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0));
            Assert.Equal(svn.ParentVersion, new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0));
        }

        [Fact]
        public void WithDashesAndUnderscoresNameOnly()
        {
            var svn = SemVerNameParser.Parse("_Name-one_two");
            Assert.Equal(svn.Label, "_Name-one_two");
            Assert.Equal(svn.Name, "_name-one_two");
            Assert.Equal(svn.NameVersion, new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0));
            Assert.Equal(svn.ParentVersion, new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0));
        }

        [Fact]
        public void OneDigit()
        {
            var svn = SemVerNameParser.Parse("name-2");
            Assert.Equal(svn.Label, "name-2");
            Assert.Equal(svn.Name, "name");
            Assert.Equal(svn.NameVersion, new Tuple<int, int, int, int, int, int>(2, 0, 0, 0, 0, 0));
            Assert.Equal(svn.ParentVersion, new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0));
        }

        [Fact]
        public void TwoDigits()
        {
            var svn = SemVerNameParser.Parse("name-21");
            Assert.Equal(svn.Label, "name-21");
            Assert.Equal(svn.Name, "name");
            Assert.Equal(svn.NameVersion, new Tuple<int, int, int, int, int, int>(21, 0, 0, 0, 0, 0));
            Assert.Equal(svn.ParentVersion, new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0));
        }

        [Fact]
        public void TwoPlaces()
        {
            var svn = SemVerNameParser.Parse("Name-2.1");
            Assert.Equal(svn.Label, "Name-2.1");
            Assert.Equal(svn.Name, "name");
            Assert.Equal(svn.NameVersion, new Tuple<int, int, int, int, int, int>(2, 1, 0, 0, 0, 0));
            Assert.Equal(svn.ParentVersion, new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0));
        }

        [Fact]
        public void TwoDigitsTwoPlaces()
        {
            var svn = SemVerNameParser.Parse("name-21.11");
            Assert.Equal(svn.Label, "name-21.11");
            Assert.Equal(svn.Name, "name");
            Assert.Equal(svn.NameVersion, new Tuple<int, int, int, int, int, int>(21, 11, 0, 0, 0, 0));
            Assert.Equal(svn.ParentVersion, new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0));
        }

        [Fact]
        public void ThreePlaces()
        {
            var svn = SemVerNameParser.Parse("name-2.1.1");
            Assert.Equal(svn.Label, "name-2.1.1");
            Assert.Equal(svn.Name, "name");
            Assert.Equal(svn.NameVersion, new Tuple<int, int, int, int, int, int>(2, 1, 1, 0, 0, 0));
            Assert.Equal(svn.ParentVersion, new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0));
        }

        [Fact]
        public void TwoDigitsThreePlaces()
        {
            var svn = SemVerNameParser.Parse("name-21.2.234");
            Assert.Equal(svn.Label, "name-21.2.234");
            Assert.Equal(svn.Name, "name");
            Assert.Equal(svn.NameVersion, new Tuple<int, int, int, int, int, int>(21, 2, 234, 0, 0, 0));
            Assert.Equal(svn.ParentVersion, new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0));
        }

        [Fact]
        public void PreReleaseAlpha()
        {
            var svn = SemVerNameParser.Parse("name-21.2.234-alpha");
            Assert.Equal(svn.Label, "name-21.2.234-alpha");
            Assert.Equal(svn.Name, "name");
            Assert.Equal(svn.NameVersion, new Tuple<int, int, int, int, int, int>(21, 2, 234, -3, 0, 0));
            Assert.Equal(svn.ParentVersion, new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0));
        }

        [Fact]
        public void PreReleaseBeta()
        {
            var svn = SemVerNameParser.Parse("name-21.2.234-beta.1");
            Assert.Equal(svn.Label, "name-21.2.234-beta.1");
            Assert.Equal(svn.Name, "name");
            Assert.Equal(svn.NameVersion, new Tuple<int, int, int, int, int, int>(21, 2, 234, -2, 1, 0));
            Assert.Equal(svn.ParentVersion, new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0));
        }

        [Fact]
        public void PreReleaseRc()
        {
            var svn = SemVerNameParser.Parse("name-21.2.234-rc.11");
            Assert.Equal(svn.Label, "name-21.2.234-rc.11");
            Assert.Equal(svn.Name, "name");
            Assert.Equal(svn.NameVersion, new Tuple<int, int, int, int, int, int>(21, 2, 234, -1, 11, 0));
            Assert.Equal(svn.ParentVersion, new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0));
        }

        [Fact]
        public void WithParent()
        {
            var svnP = SemVerNameParser.Parse("parent-1.2.3-rc.4");
            var svn = SemVerNameParser.Parse("name-21.2.234-beta.1", svnP);
            Assert.Equal(svn.Label, "name-21.2.234-beta.1");
            Assert.Equal(svn.Name, "name");
            Assert.Equal(svn.NameVersion, new Tuple<int, int, int, int, int, int>(21, 2, 234, -2, 1, 0));
            Assert.Equal(svn.ParentVersion, new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0));
        }
    }
}
