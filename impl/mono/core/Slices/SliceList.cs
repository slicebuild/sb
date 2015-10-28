using System;
using System.Collections;
using System.Collections.Generic;
using sb.Core.Utils;

namespace sb.Core.Slices
{
    public class SliceList : IEnumerable<Slice>
    {
        private readonly List<Slice> _slices = new List<Slice>();
        public IList<SemVerInfo> MissingInfos { get; } = new List<SemVerInfo>();

        public int Count => _slices.Count;
        public Slice this[int index] => _slices[index];

        public void Add(Slice slice)
        {
            if (_slices.Contains(slice))
                throw new InvalidOperationException("Duplicate slice");
            _slices.Add(slice);
        }

        public void CheckDepInfos()
        {
            foreach (var slice in _slices)
            {
                FindSlices(slice.DepInfos.ToArray());
            }
        }

        public IList<Slice> FindSimilar(IEnumerable<SemVerInfo> infos)
        {
            var list = new List<Slice>();
            foreach (var info in infos)
            {
                var slice = _slices.Find(item => item.Info.Name.Contains(info.Name));
                if (slice != null)
                    list.Add(slice);
            }
            return list;
        }

        public Slice FindSlice(SemVerInfo info)
        {
            var slice = _slices.Find(item => item.Info.Name == info.Name && item.Info.CompareByNameSemVer(info) >= 0);
            if (slice == null)
            {
                if (!MissingInfos.Contains(info))
                    MissingInfos.Add(info);
                slice = new MissingSlice(info);
            }
            return slice;
        }

        public IList<Slice> FindSlices(IEnumerable<SemVerInfo> infos)
        {
            var slices = new List<Slice>();
            foreach (var info in infos)
            {
                var slice = FindSlice(info);
                if (!slices.Contains(slice))
                {
                    slices.Add(slice);
                }
            }
            return slices;
        }

        public void Sort()
        {
            _slices.Sort((x, y) => y.Info.CompareTo(x.Info));
        }

        public IEnumerator<Slice> GetEnumerator()
        {
            return _slices.GetEnumerator();
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return ((IEnumerable) _slices).GetEnumerator();
        }
    }
}