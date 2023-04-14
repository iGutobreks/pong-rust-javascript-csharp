using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using System.Threading;
using System.Runtime.InteropServices;

namespace game
{
    public partial class Form1 : Form
    {
        [DllImport("user32.dll")]
        static extern short GetAsyncKeyState(System.Windows.Forms.Keys vKey);

        public Form1()
        {
            InitializeComponent();
        }

        private struct StateOperation
        {
            public int player1_pos;
            public int player2_pos;
            public Point ball_pos;
            public string colision_side;
            public bool ball_colised;
            public int player1_point;
            public int player2_point;
        }

        private Panel player1 = new Panel();
        private Panel player2 = new Panel();
        private Panel ball = new Panel();
        private Label display = new Label();

        private StateOperation state = new StateOperation();

        private void ball_event()
        {
            string[] player_point = display.Text.Split('|');

            char[] remove = player_point[1].ToCharArray();

            for (int i = 3; i < remove.Length; i++)
               player_point[1] = player_point[1].Replace(remove[i], ' ');

            if (!int.Parse(player_point[0].Trim()).Equals(state.player1_point)
                || !int.Parse(player_point[1].Trim()).Equals(state.player2_point))
                reset_game();

            bool ball_colision()
            {
                if (ball.Bounds.IntersectsWith(player2.Bounds))
                {
                    state.colision_side = "left";
                    randomizerY();
                }

                if (ball.Bounds.IntersectsWith(player1.Bounds))
                {
                    state.colision_side = "right";
                    randomizerY();
                }

                if (ball.Bounds.X > player2.Bounds.X)
                    state.player1_point += 1;
                if (ball.Bounds.X < player1.Bounds.X)
                    state.player2_point += 1;
                
                return false;
            }

            if (state.colision_side == "right")
            {
                state.ball_pos.X = state.ball_pos.X + defaultValues.BALL_SPEED;
                ball.Location = new Point(state.ball_pos.X, state.ball_pos.Y);
            }

            if (state.colision_side == "left")
            {
                state.ball_pos.X = state.ball_pos.X - defaultValues.BALL_SPEED;
                ball.Location = new Point(state.ball_pos.X, state.ball_pos.Y);
            }

            state.ball_colised = ball_colision();
        }

        private void player_event()
        {
            if (GetAsyncKeyState(Keys.W) < 0)
                if (player1.Location.Y >= 0)
                    state.player1_pos = state.player1_pos - defaultValues.PLAYER_SPEED;

            if (GetAsyncKeyState(Keys.S) < 0)
                if (player1.Location.Y <= (defaultValues.WINDOW_SIZE_HEITGH - player1.Size.Height))
                    state.player1_pos = state.player1_pos + defaultValues.PLAYER_SPEED;

            state.player2_pos = state.ball_pos.Y;

            //if (player2.Location.Y >= 0 && player1.Location.Y <= defaultValues.WINDOW_SIZE_HEITGH)
            //    state.player2_pos = state.player2_pos - defaultValues.PLAYER_SPEED;
            //else if (player2.Location.Y <= (defaultValues.WINDOW_SIZE_HEITGH - player2.Size.Height))
            //    state.player2_pos = state.player2_pos + defaultValues.PLAYER_SPEED;
        }

        private void reset_game()
        {
            state.player1_pos = defaultValues.WINDOW_SIZE_HEITGH / 2;
            state.player2_pos = defaultValues.WINDOW_SIZE_HEITGH / 2;
            state.ball_pos = new Point(defaultValues.WINDOW_SIZE_WIDTH / 2, defaultValues.WINDOW_SIZE_HEITGH / 2);
            display.Text = $"{state.player1_point} | {state.player2_point}";
        }

        private void randomizerY()
        {
            Random rand = new Random();
            int randVal = rand.Next(30);

            int val = randVal < 15 ? -randVal : +randVal;

            if (state.ball_pos.Y > defaultValues.WINDOW_SIZE_HEITGH - ball.Size.Width 
                ||state.ball_pos.Y > defaultValues.WINDOW_SIZE_HEITGH - ball.Size.Height)
                state.ball_pos.Y = state.ball_pos.Y - randVal;
            else
                state.ball_pos.Y = state.ball_pos.Y + val++;
        }

        private void Form1_Load(object sender, EventArgs e)
        {
            this.Text = "game";
            this.timer1.Interval = 1;

            this.BackColor = Color.Black;
            this.Size = new Size(defaultValues.WINDOW_SIZE_WIDTH, defaultValues.WINDOW_SIZE_HEITGH);

            state.player1_pos = defaultValues.WINDOW_SIZE_HEITGH / 2;
            state.player2_pos = defaultValues.WINDOW_SIZE_HEITGH / 2;
            state.ball_pos = new Point(defaultValues.WINDOW_SIZE_WIDTH / 2, defaultValues.WINDOW_SIZE_HEITGH / 2);
            state.colision_side = "right";

            display = new Label()
            {
                Text = "0 | 0",
                ForeColor = Color.White,
                Font = new Font("Arial", 10),
                Location = new Point(defaultValues.WINDOW_SIZE_WIDTH / 2, 10),
                Size = new Size(100, 100),
            };

            player1 = new Panel()
            {
                Size = new Size(10, defaultValues.PLAYER_HEIGTH),
                BackColor = Color.White,
                Location = new Point(0, 0),
            };

            player2 = new Panel()
            {
                Size = new Size(10, defaultValues.PLAYER_HEIGTH),
                BackColor = Color.White,
                Location = new Point(defaultValues.WINDOW_SIZE_WIDTH - 30, 0),
            };

            ball = new Panel()
            {
                Size = new Size(30, 30),
                BackColor = Color.White,
                Location = new Point(0, 0),
            };

            Controls.Add(player1);
            Controls.Add(player2);
            Controls.Add(ball);
            Controls.Add(display);
        }

        private void timer1_Tick(object sender, EventArgs e)
        {
            player1.Location = new Point(0, state.player1_pos);
            player2.Location = new Point(defaultValues.WINDOW_SIZE_WIDTH - player2.Size.Width, state.player2_pos);

            ball_event();
            player_event();

            //Console.WriteLine($"|{state.player1_pos}|{state.player2_pos}|{state.ball_pos}|{state.colision_side}|{state.ball_colised}");
        }
    }
}
