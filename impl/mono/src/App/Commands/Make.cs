using System.Collections.Generic;
using System.IO;
using System.Text;
using cb.Formatters;
using cb.Formatters.Docker;
using cb.Layers;
using cb.Slices;

namespace cb.App.Commands
{
    public class Make : ICommand
    {
        public Make(Args args)
        {
            Args = args;
        }

        public Args Args { get; }

        void ICommand.Run()
        {
            var layer = MakeLayer();
            var path = MakePath(layer);
            Write(layer, path);
        }

        public virtual Layer MakeLayer()
        {
            var layerParams = Args.GetLayerParams();
            var osParam = Args.GetOsParam();

            var list = new List<Slice>();

            var dirList = new SliceDirectoryList(Args.SlicesDir);
            dirList.ForEach(dir => list.AddRange(dir.FindByOs(osParam)));

            var layers = new LayerList(list);
            var layer = layers.FindLayer(layerParams[0]); //todo: check layer found and has good dependencies
            return layer;
        }

        public virtual string MakePath(Layer layer)
        {
            var path = Path.Combine(Args.EnvDir, layer.SemName.ToString()); //todo:include slices semver
            Directory.CreateDirectory(path);

            var f = new FormatterFactory().GetFormatter(Args);
            if (f.GetType() == typeof (FormatterDocker))
            {
                return Path.Combine(path, "Dockerfile");
            }

            return Path.Combine(path, layer.SemName.ToString());
        }

        public virtual void Write(Layer layer, string path)
        {
            var formatter = new FormatterFactory().GetFormatter(Args);
            Write(layer, path, formatter);
        }

        public virtual void Write(Layer layer, string path, IFormatter formatter)
        {
            var lines = new List<string>();
            Write(layer, lines);

            var slice = new Slice(layer.SemName, lines);

            var sb = new StringBuilder();
            formatter.Write(slice, sb);

            using (var fs = File.OpenWrite(path))
            using (var swr = new StreamWriter(fs))
            {
                swr.WriteLine(sb.ToString());
                swr.Flush();
            }
        }

        public virtual void Write(Layer layer, List<string> lines)
        {
            foreach (var dep in layer.Dependencies)
            {
                Write(dep, lines);
                dep.Written = true;
            }

            if (!layer.Written)
            {
                foreach (var section in layer.Sections)
                {
                    lines.Add(section.SectionType.ToString());
                    lines.AddRange(section.Lines);
                }
            }
        }        
    }
}