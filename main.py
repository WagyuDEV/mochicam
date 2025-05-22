#!/bin/python3

import threading
import cv2
import discord
from discord.ext import commands, tasks
import time
import os

#cam = cv2.VideoCapture(0, cv2.CAP_V4L2)

intents = discord.Intents.default()
intents.message_content = True
bot = commands.Bot(command_prefix='>', intents=intents)

@tasks.loop(minutes=10.0)
async def period():
    cam = cv2.VideoCapture(0, cv2.CAP_V4L2)

    for _ in range(5):
        ret, frame = cam.read()
    if ret:
        cv2.imwrite("mochi.jpg", frame)
        await bot.get_channel(CHANNEL_ID).send(file=discord.File("mochi.jpg"))
    else:
        print("Failed")
    cam.release()

@bot.command()
async def mochi(ctx):
    cam = cv2.VideoCapture(0, cv2.CAP_V4L2)

    for _ in range(5):
        ret, frame = cam.read()
    if ret:
        cv2.imwrite("mochi.jpg", frame)
        await ctx.send(file=discord.File("mochi.jpg"))
    else:
        print("Failed")
    cam.release()

@bot.event
async def on_ready():
    print("started <3")
    period.start()

bot.run('DISCORD TOKEN')

