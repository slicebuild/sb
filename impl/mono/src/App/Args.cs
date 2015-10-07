using System;
using System.Collections.Generic;
using System.IO;
using System.Reflection;
using System.Threading;
using sb.Utils;
using sb.App.Commands;

namespace sb.App
{
    public sealed class Args
    {
        public static class Options
        {
            public const string Format = "f"; // sh[ell] or d[ocker]
            public const string OutDir = "o"; // output dir
            public const string Url = "url";  // url for the slices zip file          
        }

        public static class OptionValues
        {
            public const string FormatDocker = "d";
            public const string FormatShell = "sh";
            public const string OutDir = "o"; 
            public const string Url = "url";         
        }

        public string RootDir { get; }
        public string EnvDir { get; }
        public string SlicesDir { get; }
        public ICommand Command { get; }

        private readonly Dictionary<string, string> _options = new Dictionary<string, string>();
        private string _command;
        private readonly List<string> _params = new List<string>();
        private readonly HashSet<int> _taken = new HashSet<int>();

        public Args(string[] args)
        {
            ParseOptions(args);
            ParseCommandAndParams(args);

            RootDir = Path.GetDirectoryName(Assembly.GetExecutingAssembly().Location);
            RootDir = RootDir ?? "";
            EnvDir = Path.Combine(RootDir, ".cb");
            SlicesDir = Path.Combine(EnvDir, "slices");
            Command = FindCommand(_command);
        }        

        private void ParseOptions(string[] args)
        {
            for (var i = 0; i < args.Length; i++)
            {
                if (i >= args.Length)
                    break;

                var arg = args[i];
                if (arg.StartsWith("-"))
                {
                    _taken.Add(i);

                    while (arg.StartsWith("-"))
                        arg = arg.Substring(1, arg.Length - 1).Trim();

                    var fields = typeof (Options).GetFields(BindingFlags.Static | BindingFlags.Public);
                    foreach (var field in fields)
                    {
                        var v = (string) field.GetValue(null);
                        i = ParseOptionValue(args, i, arg, v);
                    }
                }
            }
        }

        private int ParseOptionValue(string[] args, int i, string arg, string optionName)
        {
            if (!arg.StartsWith(optionName))
                return i;

            var name = "";
            var value = "";

            if (arg == optionName)
            {
                name = optionName;
                if (i < args.Length - 2)
                {
                    i++;
                    value = args[i];
                    _taken.Add(i);
                }
            }
            else if (arg.StartsWith($"{optionName}="))
            {
                name = optionName;
                value = arg.Replace($"{optionName}=", "");
            }

            _options.Add(name, value.TrimQuotes());
            return i;
        }

        private void ParseCommandAndParams(string[] args)
        {
            for (var i = 0; i < args.Length; i++)
            {
                if (_taken.Contains(i))
                    continue;

                if (_command == null)
                {
                    _command = args[i];
                    continue;
                }

                _params.Add(args[i].TrimQuotes());
            }
        }

        private ICommand FindCommand(string name)
        {
            var map = new Dictionary<string, Type>
            {
                {typeof (Fetch).Name.ToLower(), typeof (Fetch)},
                {typeof (Find).Name.ToLower(), typeof (Find)},
                {typeof (Make).Name.ToLower(), typeof (Make)},
                {typeof (Test).Name.ToLower(), typeof (Test)}
            };

            name = name?.Trim().ToLower() ?? "";
            if (map.ContainsKey(name))
            {
                var command = Activator.CreateInstance(map[name], 
                    BindingFlags.CreateInstance | BindingFlags.Public | BindingFlags.Instance, 
                    null, 
                    new object[] {this},
                    Thread.CurrentThread.CurrentCulture);
                return (ICommand) command;
            }

            return new Help(this);
        }

        public string GetOption(string name, string def)
        {
            return _options.ContainsKey(name) ? _options[name] : def;
        }

        public void SetOption(string name, string value)
        {
            if (_options.ContainsKey(name))
                _options[name] = value;
            else
                _options.Add(name, value);
        }

        public string GetParam(int pos, string def)
        {
            return _params.Count > pos ? _params[pos] : def;
        }

        public string[] GetLayerParams()
        {
            var layerParam = GetParam(0, null);
            layerParam = layerParam ?? "jekyll"; //todo: change the default to the waybuild image
            return layerParam.Split(",");
        }

        public string GetOsParam()
        {
            var osParam = GetParam(1, null);
            return osParam ?? "debian";
        }
    }
}