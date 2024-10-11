'use client';
import classes from './home.module.css';
import { Image, Container, Title, Button, Group, Text, List, ThemeIcon, rem } from '@mantine/core';
import { IconCheck } from '@tabler/icons-react';
import { useRouter } from 'next/navigation';

export default function Home() {

    const router = useRouter();

  return (
      <Container size="md">
          <div className={classes.inner}>
              <div className={classes.content}>
                  <Title className={classes.title}>
                      A <span className={classes.highlight}>modern</span> platform <br /> to practise coding
                  </Title>
                  <Text c="dimmed" mt="md">
                      Built to provide an universal application for tutors and students to manage coding assignments. It is an easy and
                      practical way to practise coding and patterns for exams and tests.
                  </Text>

                  <List
                      mt={30}
                      spacing="sm"
                      size="sm"
                      icon={
                          <ThemeIcon size={20} radius="xl">
                              <IconCheck style={{ width: rem(12), height: rem(12) }} stroke={1.5} />
                          </ThemeIcon>
                      }
                  >
                      <List.Item>
                          <b>Question assignments</b> – Besides coding assignments, you can also create assignments with questions that can be answered.
                      </List.Item>
                      <List.Item>
                          <b>Free and open source</b> – The project is community driven and can be used for free. Everyone can view the source code.
                      </List.Item>
                      <List.Item>
                          <b>Code testing</b> – Tutors can create code tests to validate if the code works as expected.
                      </List.Item>
                  </List>

                  <Group mt={30}>
                      <Button radius="xl" size="md" className={classes.control} onClick={() => router.push('/register')}>
                          Get started
                      </Button>
                      <Button variant="default" radius="xl" size="md" className={classes.control} onClick={() => router.push('https://github.com/MathisBurger/CodeCanvas')}>
                          Source code
                      </Button>
                  </Group>
              </div>
              <Image src="image.svg" className={classes.image} />
          </div>
      </Container>
  );
}
