﻿using System.Collections.Generic;
using System.Linq;
using System.Text;
using sb.Core.Slices;

namespace sb.Core.Layers
{
    public class Layer
    {
        public Layer(SliceList sliceList, Slice slice)
        {
            SliceList = sliceList;
            Slice = slice;
        }

        public override bool Equals(object obj)
        {
            var other = (Layer)obj;
            return Slice.Equals(other.Slice);
        }

        public override int GetHashCode()
        {
            return Slice.GetHashCode();
        }

        public override string ToString()
        {
            return Slice.ToString();
        }

        public SliceList SliceList { get; }
        public Slice Slice { get; }
        public bool Written { get; private set; }

        protected List<Layer> Dependencies { get; } = new List<Layer>();

        public void InsertDependency(int pos, Layer layer)
        {
            Dependencies.Insert(pos, layer);
        }

        public virtual void FindDependenciesRecursive(LayerList layerList)
        {
            foreach (var depInfo in Slice.DepInfos)
            {
                var layer = layerList.FindLayer(depInfo);
                if (!Dependencies.Contains(layer))
                    Dependencies.Add(layer);
            }

            foreach (var dep in Dependencies)
            {
                dep.FindDependenciesRecursive(layerList);
            }
        }

        public virtual void Write(IList<string> lines)
        {
            if (Written)
                return;

            for (var i = Dependencies.Count - 1; i >= 0; i--)
            {
                Dependencies[i].Write(lines);
            }

            foreach (var section in Slice.Sections.Where(s => s.SectionType != SliceSection.Type.DEP))
            {
                lines.Add("");
                lines.Add($"# {Slice.Info}");
                lines.Add(section.SectionType.ToString());
                foreach (var line in section.Lines)
                {
                    lines.Add(line);
                }
            }

            Written = true;
        }      
    }
}