using System;

namespace SampleNamespace
{
    public class Person
    {
        public string Name { get; set; }
        public int Age { get; set; }

        public Person(string name, int age)
        {
            Name = name;
            Age = age;
        }

        public void Introduce()
        {
            Console.WriteLine($"Hi, I'm {Name} and I'm {Age} years old.");
        }
    }

    public interface IVehicle
    {
        void Start();
        void Stop();
    }

    public class Car : IVehicle
    {
        public string Model { get; set; }

        public Car(string model)
        {
            Model = model;
        }

        public void Start()
        {
            Console.WriteLine($"{Model} car started.");
        }

        public void Stop()
        {
            Console.WriteLine($"{Model} car stopped.");
        }
    }

    public struct Point
    {
        public int X { get; set; }
        public int Y { get; set; }

        public Point(int x, int y)
        {
            X = x;
            Y = y;
        }
    }

    public static class MathHelper
    {
        public static int Add(int a, int b)
        {
            return a + b;
        }
    }

    public enum DaysOfWeek
    {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }
}
