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

        public Slice this[int index] => _slices[index];

        public int Count => _slices.Count;

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
                var slice = _slices.Find(item => item.SemVerInfo.Name.Contains(info.Name));
                if (slice != null)
                    list.Add(slice);
            }
            return list;
        }

        public Slice FindSlice(SemVerInfo svi)
        {
            var slice = _slices.Find(item => item.SemVerInfo.Name == svi.Name && item.SemVerInfo.CompareByNameSemVer(svi) >= 0);
            if (slice == null)
            {
                if (!MissingInfos.Contains(svi))
                    MissingInfos.Add(svi);
                slice = new MissingSlice(svi);
            }
            return slice;
        }

        public IList<Slice> FindSlices(SemVerInfo[] svis)
        {
            var slices = new List<Slice>();
            foreach (var svi in svis)
            {
                var slice = FindSlice(svi);
                if (!slices.Contains(slice))
                {
                    slices.Add(slice);
                }
            }
            return slices;
        }

        public void Sort()
        {
            _slices.Sort((x, y) => y.SemVerInfo.CompareTo(x.SemVerInfo));
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