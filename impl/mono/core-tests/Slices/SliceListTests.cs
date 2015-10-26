using System;
using System.Collections.Generic;
using sb.Core.Slices;
using sb.Core.Utils;
using Xunit;

namespace sb.Core.Tests.Slices
{
    public class SliceListTests
    {
        [Fact]
        public void FindsSlice()
        {
            var sviB = new SemVerInfo("slices-1");
            var sliceList = new SliceList();
            sliceList.Add(new Slice(null, new SemVerInfo(sviB.NameSemVer, "item1"), new List<string>()));
            sliceList.Add(new Slice(null, new SemVerInfo(sviB.NameSemVer, "item2"), new List<string>()));
            sliceList.Add(new Slice(null, new SemVerInfo(sviB.NameSemVer, "item3"), new List<string>()));
            var slice = sliceList.FindSlice(new SemVerInfo("item1"));
            Assert.NotEqual(slice.GetType(), typeof(MissingSlice));
        }

        [Fact]
        public void NotFindsSlice()
        {
            var sviB = new SemVerInfo("slices-1");
            var sliceList = new SliceList();
            sliceList.Add(new Slice(null, new SemVerInfo(sviB.NameSemVer, "item1"), new List<string>()));
            sliceList.Add(new Slice(null, new SemVerInfo(sviB.NameSemVer, "item2"), new List<string>()));
            sliceList.Add(new Slice(null, new SemVerInfo(sviB.NameSemVer, "item3"), new List<string>()));
            var slice = sliceList.FindSlice(new SemVerInfo("item4"));
            Assert.Equal(slice.GetType(), typeof(MissingSlice));
        }

        [Fact]
        public void SortsOneBunch()
        {
            var sviB = new SemVerInfo("slices-1");
            var sliceList = new SliceList();
            sliceList.Add(new Slice(null, new SemVerInfo(sviB.NameSemVer, "item1-1"), new List<string>()));
            sliceList.Add(new Slice(null, new SemVerInfo(sviB.NameSemVer, "item1-2"), new List<string>()));
            sliceList.Add(new Slice(null, new SemVerInfo(sviB.NameSemVer, "item2"), new List<string>()));
            sliceList.Add(new Slice(null, new SemVerInfo(sviB.NameSemVer, "item3"), new List<string>()));
            sliceList.Sort();

            Assert.Equal(sliceList[0].SemVerInfo.NameMajor, 2);
        }

        [Fact]
        public void SortsTwoBunches()
        {
            var sviB1 = new SemVerInfo("slices-1");
            var sviB2 = new SemVerInfo("slices-2");
            var sliceList = new SliceList();

            sliceList.Add(new Slice(null, new SemVerInfo(sviB1.NameSemVer, "item1-1"), new List<string>()));
            sliceList.Add(new Slice(null, new SemVerInfo(sviB1.NameSemVer, "item1-2"), new List<string>()));
            sliceList.Add(new Slice(null, new SemVerInfo(sviB1.NameSemVer, "item2"), new List<string>()));
            sliceList.Add(new Slice(null, new SemVerInfo(sviB1.NameSemVer, "item3"), new List<string>()));

            sliceList.Add(new Slice(null, new SemVerInfo(sviB2.NameSemVer, "item1-1"), new List<string>()));
            sliceList.Add(new Slice(null, new SemVerInfo(sviB2.NameSemVer, "item1-2"), new List<string>()));
            sliceList.Add(new Slice(null, new SemVerInfo(sviB2.NameSemVer, "item2"), new List<string>()));
            sliceList.Add(new Slice(null, new SemVerInfo(sviB2.NameSemVer, "item3"), new List<string>()));

            sliceList.Sort();

            Assert.Equal(sliceList[0].SemVerInfo.BunchSemVer.Item1, 2);
            Assert.Equal(sliceList[0].SemVerInfo.NameMajor, 2);
        }

        [Fact]
        public void TracksMissing()
        {
            var sviB = new SemVerInfo("slices-1");
            var sliceList = new SliceList();
            sliceList.Add(new Slice(null, new SemVerInfo(sviB.NameSemVer, "item1-1"), new List<string>()));
            sliceList.Add(new Slice(null, new SemVerInfo(sviB.NameSemVer, "item1-2"), new List<string>()));
            sliceList.Add(new Slice(null, new SemVerInfo(sviB.NameSemVer, "item2"), new List<string>()));
            sliceList.Add(new Slice(null, new SemVerInfo(sviB.NameSemVer, "item3"), new List<string>()));
            sliceList.Sort();

            var svi = new SemVerInfo("item1-2.1");
            var slice = sliceList.FindSlice(svi);
            Assert.Equal(slice.GetType(), typeof(MissingSlice));
            Assert.Equal(sliceList.MissingInfos[0], svi);
        }

        [Fact]
        public void ThrowsOnDuplicate()
        {
            var sviB = new SemVerInfo("slices-1");
            var sliceList = new SliceList();
            sliceList.Add(new Slice(null, new SemVerInfo(sviB.NameSemVer, "item1-1"), new List<string>()));
            var ex = Assert.Throws<InvalidOperationException>(
                () => sliceList.Add(new Slice(null, new SemVerInfo(sviB.NameSemVer, "item1-1"), new List<string>())));
            Assert.NotNull(ex);
        }
    }
}